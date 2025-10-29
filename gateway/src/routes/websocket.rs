use std::net::SocketAddr;

use axum::{extract::{ws::{Message, WebSocket}, ConnectInfo, WebSocketUpgrade}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{services::{self, matchmaking_service::AddToQueueRequestPb}, utils::claim::UserClaims};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ClientMessage {
    Events(EventData),
    Chat(ChatData),
    Ping,

    #[serde(rename = "matchmaking:add-to-queue")]
    MatchMakingAddToQueue(AddToQueueDto),
}

#[derive(Debug, Deserialize)]
struct EventData {
    event_type: String,
    payload: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ChatData {
    room: String,
    message: String,
}

#[derive(Debug, Deserialize)]
struct AddToQueueDto {
    #[serde(rename = "gameType")]
    game_type: String,
    ranked: bool,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "data")]
enum ServerMessage {
    EventAck { received: bool },
    ChatAck { delivered: bool },
    Pong,
    Ack,
}

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    user: UserClaims,
) -> impl IntoResponse {
    tracing::info!("Handling socket connection for user_id: {}", user.uid);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, user.uid))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, _who: SocketAddr, user_id: Uuid) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            tracing::debug!("raw message on {}", text.clone());
            match serde_json::from_str::<ClientMessage>(&text) {
                Ok(client_msg) => {
                    match client_msg {
                        ClientMessage::Events(event_data) => {
                            handle_events(event_data, &mut socket).await;
                        }
                        ClientMessage::Chat(chat_data) => {
                            handle_chat(chat_data, &mut socket).await;
                        }
                        ClientMessage::Ping => {
                            handle_ping(&mut socket).await;
                        }

                        ClientMessage::MatchMakingAddToQueue(data) => {
                            handle_add_to_queue(data, user_id, &mut socket).await;
                        }
                    }
                }
                Err(_) => {
                    let _ = socket.send(Message::Text("Invalid message format".to_string())).await;
                }
            }
        }
    }
}

async fn handle_events(event_data: EventData, socket: &mut WebSocket) {
    println!("Received event: {:?}", event_data.event_type);
    println!("Received event data: {:?}", event_data.payload);

    let response = ServerMessage::EventAck { received: true };
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}

async fn handle_chat(chat_data: ChatData, socket: &mut WebSocket) {
    println!("Chat in room {}: {}", chat_data.room, chat_data.message);

    let response = ServerMessage::ChatAck { delivered: true };
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}

async fn handle_ping(socket: &mut WebSocket) {
    let response = ServerMessage::Pong;
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}

async fn handle_add_to_queue(add_to_queue_data: AddToQueueDto, account_id: Uuid, socket: &mut WebSocket) {
    println!("Add to queue game_type: {}, ranked: {}", add_to_queue_data.game_type, add_to_queue_data.ranked);

    services::matchmaking_service::add_to_queue(&AddToQueueRequestPb{
        account_id,
        game_type: add_to_queue_data.game_type,
        ranked: add_to_queue_data.ranked,
    })
    .await
    .expect("Failed to add player to queue");

    let response = ServerMessage::Ack;
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}
