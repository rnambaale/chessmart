use socketioxide::extract::{SocketRef, State};
use tracing::{info, warn};

use crate::{constants::ACCESS_TOKEN_DECODE_KEY, server::state::AppState, utils::claim::UserClaims};

mod chat;
mod game;
mod matchmaking;

pub async fn on_connect(socket: SocketRef) {
    println!("Socket connected: {}", socket.id);

    socket.on("matchmaking:add-to-queue", crate::handlers::matchmaking::handle_add_to_queue);
    socket.on("matchmaking:remove-from-queue", crate::handlers::matchmaking::handle_remove_from_queue);
    socket.on("matchmaking:accept-pending-game", crate::handlers::matchmaking::handle_accept_pending_game);
    socket.on("matchmaking:join-game", crate::handlers::matchmaking::handle_join_game);
    socket.on("matchmaking:join-lobby", crate::handlers::matchmaking::handle_join_lobby);
    socket.on("matchmaking:leave-lobby", crate::handlers::matchmaking::handle_leave_lobby);

    socket.on("game:make-move", crate::handlers::game::handle_make_move);
    socket.on("game:resign", crate::handlers::game::handle_resign);
    socket.on("game:check-result", crate::handlers::game::handle_check_result);

    socket.on("chat:send-message", crate::handlers::chat::handle_send_message);
}

/// Handles the connection of a new user.
/// Be careful to not emit anything to the user before the authentication is done.
pub async fn authenticate_middleware(
    socket: SocketRef,
    State(state): State<AppState>,
) -> Result<(), anyhow::Error> {
    let jwt = if let Some(cookie_header) = socket.req_parts().headers.get("cookie")
        .and_then(|value| value.to_str().ok())
    {
        get_cookie_value("jwt", Some(cookie_header))
    } else {
        None
    };

    let jwt = match jwt {
        Some(jwt) => jwt,
        None => {
            warn!("Client tried to connect without JWT");
            let _ = socket.emit(
                "socket:missing-jwt",
                &serde_json::json!({
                    "message": "Unauthorized"
                }));
            socket.disconnect().ok();
            return Ok(());
        }
    };

    // Verify JWT
    let jwt_data = match UserClaims::decode(&jwt, &ACCESS_TOKEN_DECODE_KEY) {
        Ok(data) => data.claims,
        Err(err) => {
            warn!("Client tried to connect with invalid JWT: {}", err);
            let _ = socket.emit("socket:missing-jwt", &serde_json::json!({
                "message": "Unauthorized"
            }));
            socket.disconnect().ok();
            return Ok(());
        }
    };

    info!("Client connected with account ID: {}", jwt_data.uid);

    socket.extensions.insert(jwt_data.clone());

    socket.join(jwt_data.uid.to_string());

    let mut matchmaking_client = state.matchmaking_client.clone();

    let shared::GetAccountStatusResponse {
        status,
        ..
    } = matchmaking_client.get_account_status(
        shared::GetAccountStatusRequest {
            account_id: jwt_data.uid.to_string()
        }
    ).await.expect("Failed to get account status").into_inner();

    let _ = socket.emit("matchmaking:account-status-update", &status);

    Ok(())
}

fn get_cookie_value(cookie_name: &str, cookie_header: Option<&str>) -> Option<String> {
    let cookie_header = cookie_header?;

    cookie_header
        .split(';')
        .map(|cookie| cookie.trim())
        .find(|cookie| cookie.starts_with(&format!("{}=", cookie_name)))
        .and_then(|cookie| {
            cookie.split('=').nth(1).map(|value| value.to_string())
        })
}
