use std::sync::Arc;

use shared::{error::BunnyChessApiError, primitives::GameType};

use crate::repositories::{matchmaking_queue_repository::PlayerStatus, player_status_repository::{PlayerStatusRepository}};

// #[derive(Serialize, Deserialize)]
pub struct MatchMakingStatus {
    pub status: PlayerStatus,
    pub game_type: Option<GameType>,
    pub ranked: Option<bool>,
    pub game_id: Option<String>
}

#[async_trait::async_trait]
pub trait PlayerStatusService: Send + Sync {
    async fn get_player_status(&self, account_id: &str) -> Result<MatchMakingStatus, BunnyChessApiError>;
}

pub struct PlayerStatusServiceImpl {
    pub player_status_repository: Arc<dyn PlayerStatusRepository>,
}

impl PlayerStatusServiceImpl {
    pub fn new(player_status_repository: Arc<dyn PlayerStatusRepository>) -> Self {
        Self {
            player_status_repository,
        }
    }
}

#[async_trait::async_trait]
impl PlayerStatusService for PlayerStatusServiceImpl  {
    async fn get_player_status(&self, account_id: &str) -> Result<MatchMakingStatus, BunnyChessApiError> {
        self.player_status_repository.get_player_status(account_id).await
    }
}
