
use serde::Deserialize;
use socketioxide::extract::{AckSender, Data, SocketRef, State};

use crate::{server::state::AppState, utils::claim::UserClaims};

#[derive(Debug, Deserialize)]
pub struct MakeMoveRequestDto {
    #[serde(rename = "gameId")]
    game_id: String,
    #[serde(rename = "move")]
    game_move: String,
}

pub async fn handle_make_move(
    socket: SocketRef,
    Data(payload): Data<MakeMoveRequestDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    println!("Make move game_id: {}, move: {}", payload.game_id, payload.game_move);

    let mut game_client = state.game_client.clone();

    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;

    game_client.make_move(shared::MakeMoveRequest{
        account_id: account_id.to_string(),
        game_id: payload.game_id,
        r#move: payload.game_move,
    }).await.expect("Failed to make move.");

    ack.send(&()).ok();
}

#[derive(Debug, Deserialize)]
pub struct ResignRequestDto {
    #[serde(rename = "gameId")]
    game_id: String,
}

pub async fn handle_resign(
    socket: SocketRef,
    Data(payload): Data<ResignRequestDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    println!("Resign request, game_id: {}", payload.game_id);

    let mut game_client = state.game_client.clone();

    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;

    game_client.resign(shared::ResignRequest{
        account_id: account_id.to_string(),
        game_id: payload.game_id,
    }).await.expect("Failed to resign from game.");

    ack.send(&()).ok();
}
#[derive(Debug, Deserialize)]
pub struct CheckResultRequestDto {
    #[serde(rename = "gameId")]
    game_id: String,
}

pub async fn handle_check_result(
    Data(payload): Data<CheckResultRequestDto>,
    ack: AckSender,
    State(state): State<AppState>,
) {
    println!("Resign request, game_id: {}", payload.game_id);

    let mut game_client = state.game_client;

    game_client.check_game_result(shared::CheckGameResultRequest{
        game_id: payload.game_id,
    }).await.expect("Failed to check game result.");

    ack.send(&()).ok();
}
