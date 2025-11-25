use serde::Deserialize;
use socketioxide::extract::{AckSender, Data, SocketRef};
use uuid::Uuid;

use crate::utils::claim::UserClaims;

#[derive(Debug, Deserialize)]
pub struct SendChatMessageRequestDto {
    #[serde(rename = "gameId")]
    game_id: String,
    message: String,
}

pub async fn handle_send_message(
    socket: SocketRef,
    Data(payload): Data<SendChatMessageRequestDto>,
    ack: AckSender,
) {
    let SendChatMessageRequestDto { game_id, message  } = payload;

    let account_id = socket.extensions.get::<UserClaims>()
        .ok_or("Unauthorized")
        .unwrap()
        .uid;

    let notification = serde_json::json!({
        "message": message,
        "id": Uuid::new_v4(),
        "username": account_id, // TODO: Use the user's username
    });

    socket.to(game_id)
        .emit("chat:message", &notification)
        .await
        .expect("Failed to emit event");

    ack.send(&()).ok();
}
