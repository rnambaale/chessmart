use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStartEvent {
    pub account_id_0: String,
    pub account_id_1: String,
    pub game_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStateUpdateEvent {
    pub account_id: String,
    pub game_id: String,
    pub r#move: String,
    pub fen: String,
    pub seq: u64,
    pub clocks: ClockState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameOverEvent {
    pub account_id_0: String,
    pub account_id_1: String,
    pub outcome: String,
    pub game_over_reason: String,
    pub winner_account_id: Option<String>,
    pub game_id: String,
    pub game_type: String,
    pub metadata: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClockState {
    pub w: u32, // White time in milliseconds
    pub b: u32, // Black time in milliseconds
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    GameStart(GameStartEvent),
    GameStateUpdate(GameStateUpdateEvent),
    GameOver(GameOverEvent),
}

impl GameEvent {
    pub fn subject(&self) -> &'static str {
        match self {
            GameEvent::GameStart(_) => "bunnychess.game.game-start",
            GameEvent::GameStateUpdate(_) => "bunnychess.game.game-state-update",
            GameEvent::GameOver(_) => "bunnychess.game.game-over",
        }
    }
}
