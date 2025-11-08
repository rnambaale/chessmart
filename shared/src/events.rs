use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameStartEvent {
    pub account_id_0: String,
    pub account_id_1: String,
    pub game_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameEvent {
    GameStart(GameStartEvent),
    // GameStateUpdate(String),
    // GameOver(String),
}

impl GameEvent {
    pub fn subject(&self) -> &'static str {
        match self {
            GameEvent::GameStart(_) => "bunnychess.game.game-start",
            // GameEvent::GameStateUpdate(_) => "bunnychess.game.game-state-update",
            // GameEvent::GameOver(_) => "bunnychess.game.game-over",
        }
    }
}
