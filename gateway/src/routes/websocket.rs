use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, State, WebSocketUpgrade, ws::{Message, WebSocket}}, response::IntoResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{server::state::{AppState, MatchmakingGrpcClient}, utils::claim::UserClaims};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ClientMessage {
    Events(EventData),
    Chat(ChatData),
    Ping,

    #[serde(rename = "matchmaking:add-to-queue")]
    MatchMakingAddToQueue(AddToQueueDto),

    #[serde(rename = "matchmaking:remove-from-queue")]
    MatchMakingRemoveFromQueue(RemoveFromQueueDto),

    #[serde(rename = "matchmaking:accept-pending-game")]
    MatchMakingAcceptPendingGame(AcceptPendingGameDto),
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

#[derive(Debug, Deserialize)]
struct RemoveFromQueueDto {
    #[serde(rename = "accountId")]
    account_id: String,
}

#[derive(Debug, Deserialize)]
struct AcceptPendingGameDto {
    #[serde(rename = "pendingGameId")]
    pending_game_id: String,
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
    State(state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("Handling socket connection for user_id: {}", user.uid);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, user.uid, state))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket, _who: SocketAddr, user_id: Uuid, state: AppState) {
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
                            let matchmaking_grpc_client = state.matchmaking_client.clone();
                            handle_add_to_queue(data, user_id, &mut socket, matchmaking_grpc_client).await;
                        }

                        ClientMessage::MatchMakingRemoveFromQueue(data) => {
                            let matchmaking_grpc_client = state.matchmaking_client.clone();
                            handle_remove_from_queue(data, &mut socket, matchmaking_grpc_client).await;
                        }

                        ClientMessage::MatchMakingAcceptPendingGame(data) => {
                            let matchmaking_grpc_client = state.matchmaking_client.clone();
                            handle_accept_pending_game(data, user_id, &mut socket, matchmaking_grpc_client).await;
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

async fn handle_add_to_queue(
    payload: AddToQueueDto,
    account_id: Uuid,
    socket: &mut WebSocket,
    mut client: MatchmakingGrpcClient,
) {
    println!("Add to queue game_type: {}, ranked: {}", payload.game_type, payload.ranked);

    client.add_to_queue(shared::AddToQueueRequest{
        account_id: account_id.to_string(),
        game_type: payload.game_type,
        ranked: payload.ranked,
    }).await.expect("Failed to add player to queue.");

    let response = ServerMessage::Ack;
    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}

async fn handle_remove_from_queue(
    payload: RemoveFromQueueDto,
    socket: &mut WebSocket,
    mut client: MatchmakingGrpcClient,
) {
    println!("Remove from queue account_id: {}", payload.account_id);

    client.remove_from_queue(shared::RemoveFromQueueRequest {
        account_id: payload.account_id,
    }).await.expect("Failed to remove player from queue.");

    let response = ServerMessage::Ack;

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}

async fn handle_accept_pending_game(
    payload: AcceptPendingGameDto,
    account_id: Uuid,
    socket: &mut WebSocket,
    mut client: MatchmakingGrpcClient,
) {
    println!("Accept pending game account_id: {}", account_id);

    client.accept_pending_game(shared::AcceptPendingGameRequest {
        account_id: account_id.to_string(),
        pending_game_id: payload.pending_game_id,
    }).await.expect("Failed to remove player from queue.");

    let response = ServerMessage::Ack;

    if let Ok(json) = serde_json::to_string(&response) {
        let _ = socket.send(Message::Text(json)).await;
    }
}
