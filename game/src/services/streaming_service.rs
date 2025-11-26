use shared::{events::{GameEvent, GameOverEvent, GameStartEvent, GameStateUpdateEvent}};

use crate::{error::GameServiceError, state::state::AppState};

pub async fn emit_game_start(state: &AppState, payload: GameStartEvent) -> Result<(), GameServiceError> {
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

pub async fn emit_game_state_update(
    state: &AppState,
    payload: GameStateUpdateEvent
) -> Result<(), GameServiceError> {
    let event = GameEvent::GameStateUpdate(payload.clone());
    let subject = event.subject();

    let payload = serde_json::to_vec(&event).unwrap();

    state
        .jetstream
        .publish(subject.to_string(), payload.into())
        .await
        .expect("Failed to emit game-state-update event");

    Ok(())
}

pub async fn emit_game_over_event(
    state: &AppState,
    payload: GameOverEvent
) -> Result<(), GameServiceError> {
    let event = GameEvent::GameOver(payload.clone());
    let subject = event.subject();

    let payload = serde_json::to_vec(&event).unwrap();

    state
        .jetstream
        .publish(subject.to_string(), payload.into())
        .await
        .expect("Failed to emit game-over event");

    Ok(())
}
