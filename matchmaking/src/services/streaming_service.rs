use shared::{error::BunnyChessApiError, events::{MatchmakingEvent, PendingGameReadyEvent, PendingGameTimeoutEvent}};

use crate::state::state::AppState;

pub async fn emit_pending_game_ready(
    state: &AppState,
    payload: PendingGameReadyEvent,
) -> Result<(), BunnyChessApiError> {
    let event = MatchmakingEvent::PendingGameReady(payload.clone());
    let subject = event.subject();

    let payload = serde_json::to_vec(&event).unwrap();

    state
        .jetstream
        .publish(subject.to_string(), payload.into())
        .await
        .expect("Failed to emit pending-game-ready event");

    Ok(())
}

pub async fn emit_pending_game_timeout(
    state: &AppState,
    payload: PendingGameTimeoutEvent,
) -> Result<(), BunnyChessApiError> {
    let event = MatchmakingEvent::PendingGameTimeout(payload.clone());
    let subject = event.subject();

    let payload = serde_json::to_vec(&event).unwrap();

    state
        .jetstream
        .publish(subject.to_string(), payload.into())
        .await
        .expect("Failed to emit pending-game-timeout event");

    Ok(())
}
