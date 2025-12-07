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
    // pub fen: String,
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
            GameEvent::GameStart(_) => "chessmart.game.game-start",
            GameEvent::GameStateUpdate(_) => "chessmart.game.game-state-update",
            GameEvent::GameOver(_) => "chessmart.game.game-over",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingGameReadyEvent {
    pub account_id_0: String,
    pub account_id_1: String,
    pub pending_game_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingGameTimeoutEvent {
    pub account_id_0: String,
    pub account_id_1: String,
    pub pending_game_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EloChangeEvent {
    pub account_id: String,
    pub new_elo: i32,
    pub elo_change: i32,
    pub ranked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MatchmakingEvent {
    PendingGameReady(PendingGameReadyEvent),
    PendingGameTimeout(PendingGameTimeoutEvent),
    EloChange(EloChangeEvent),
}

impl MatchmakingEvent {
    pub fn subject(&self) -> &'static str {
        match self {
            MatchmakingEvent::PendingGameReady(_) => "chessmart.matchmaking.pending-game-ready",
            MatchmakingEvent::PendingGameTimeout(_) => "chessmart.matchmaking.pending-game-timeout",
            MatchmakingEvent::EloChange(_) => "chessmart.matchmaking.elo-change",
        }
    }
}
