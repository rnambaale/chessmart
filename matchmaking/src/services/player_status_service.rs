use std::sync::Arc;
use shared::primitives::GameType;

use crate::{error::MatchmakingServiceError, repositories::{matchmaking_queue_repository::PlayerStatus, player_status_repository::PlayerStatusRepository}};

// #[derive(Serialize, Deserialize)]
pub struct MatchMakingStatus {
    pub status: PlayerStatus,
    pub game_type: Option<GameType>,
    pub ranked: Option<bool>,
    pub game_id: Option<String>
}

#[async_trait::async_trait]
pub trait PlayerStatusServiceContract: Send + Sync {
    async fn get_player_status(&self, account_id: &str) -> Result<MatchMakingStatus, MatchmakingServiceError>;
}

pub struct PlayerStatusService {
    pub player_status_repository: Arc<dyn PlayerStatusRepository>,
}

impl PlayerStatusService {
    pub fn new(player_status_repository: Arc<dyn PlayerStatusRepository>) -> Self {
        Self {
            player_status_repository,
        }
    }
}

#[async_trait::async_trait]
impl PlayerStatusServiceContract for PlayerStatusService  {
    async fn get_player_status(&self, account_id: &str) -> Result<MatchMakingStatus, MatchmakingServiceError> {
        self.player_status_repository.get_player_status(account_id).await
    }
}
