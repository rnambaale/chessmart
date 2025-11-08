use shared::{error::BunnyChessApiError, events::{GameEvent, GameStartEvent}};

use crate::state::state::AppState;

pub async fn emit_game_start(state: &AppState, payload: GameStartEvent) -> Result<(), BunnyChessApiError> {
    let event = GameEvent::GameStart(payload.clone());
    let subject = event.subject();

    let payload = serde_json::to_vec(&event).unwrap();

    state
        .jetstream
        .publish(subject.to_string(), payload.into())
        .await
        .expect("Failed to emit game-start event");

    Ok(())
}
