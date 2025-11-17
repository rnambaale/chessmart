use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use socketioxide::extract::{AckSender, Data, SocketRef, State};

use crate::{server::state::AppState, utils::claim::UserClaims};

#[derive(Debug, Deserialize)]
pub struct AddToQueueDto {
    #[serde(rename = "gameType")]
    game_type: String,
    ranked: bool,
}

pub async fn handle_add_to_queue(
    socket: SocketRef,
    Data(payload): Data<AddToQueueDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    println!("Add to queue game_type: {}, ranked: {}", payload.game_type, payload.ranked);

    let mut matchmaking_client = state.matchmaking_client.clone();

    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;

    matchmaking_client.add_to_queue(shared::AddToQueueRequest{
        account_id: account_id.to_string(),
        game_type: payload.game_type,
        ranked: payload.ranked,
    }).await.expect("Failed to add player to queue.");

    ack.send(&()).ok();
}

#[derive(Debug, Deserialize)]
pub struct RemoveFromQueueDto {
    #[serde(rename = "accountId")]
    account_id: String,
}

pub async fn handle_remove_from_queue(
    _socket: SocketRef,
    Data(payload): Data<RemoveFromQueueDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    let mut matchmaking_client = state.matchmaking_client.clone();

    matchmaking_client.remove_from_queue(shared::RemoveFromQueueRequest {
        account_id: payload.account_id,
    }).await.expect("Failed to remove player from queue.");

    ack.send(&()).ok();
}

#[derive(Debug, Deserialize)]
pub struct AcceptPendingGameDto {
    #[serde(rename = "pendingGameId")]
    pending_game_id: String,
}

pub async fn handle_accept_pending_game(
    socket: SocketRef,
    Data(payload): Data<AcceptPendingGameDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;
    println!("Accept pending game account_id: {}", account_id);

    let mut matchmaking_client = state.matchmaking_client.clone();

    matchmaking_client.accept_pending_game(shared::AcceptPendingGameRequest {
        account_id: account_id.to_string(),
        pending_game_id: payload.pending_game_id,
    }).await.expect("Failed to accept pending game.");

    ack.send(&()).ok();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinGameDto {
    #[serde(rename = "gameId")]
    game_id: String,
}

pub async fn handle_join_game(
    socket: SocketRef,
    Data(payload): Data<JoinGameDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    let mut matchmaking_client = state.matchmaking_client.clone();
    let mut game_client = state.game_client.clone();

    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;

    let shared::GetAccountStatusResponse {
        status, game_id, ..
    } = matchmaking_client
        .get_account_status(shared::GetAccountStatusRequest {
            account_id: account_id.to_string()
        })
        .await
        .expect("Failed to get account status")
        .into_inner();

    if status == "playing" && game_id == Some(payload.game_id.to_owned()) {
        socket.join(payload.game_id.to_owned())
    }

    let shared::GetGameStateResponse { game_repr } = game_client
        .get_game_state(shared::GetGameStateRequest {
            game_id: payload.game_id.to_owned()
        })
        .await
        .expect("Failed to get game state.")
        .into_inner();

    ack.send(&game_repr).ok();
}

#[derive(Serialize)]
pub struct SerializableQueueSize {
    pub normal: u32,
    pub ranked: u32,
}

pub async fn handle_join_lobby(
    socket: SocketRef,
    State(state): State<AppState>,
) {
    let mut matchmaking_client = state.matchmaking_client.clone();

    socket.join("lobby");

    let shared::GetQueueSizesResponse {
        queue_sizes
    } = matchmaking_client.get_queue_sizes(shared::GetQueueSizesRequest {}).await
        .expect("Failed to get queue sizes")
        .into_inner();

    let mut serializable_sizes: HashMap<String, SerializableQueueSize> = HashMap::new();

    for (queue_name, queue_size) in queue_sizes {
        let shared::QueueSize { normal, ranked } = queue_size;

        serializable_sizes.insert(
            queue_name,
            SerializableQueueSize {
                normal,
                ranked
            }
        );
    }

    socket.emit("matchmaking:queue-sizes", &serializable_sizes)
        .expect("Failed to emit event");
}

pub async fn handle_leave_lobby(
    socket: SocketRef
) {
    socket.leave("lobby");
}
