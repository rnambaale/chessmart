use std::collections::HashMap;

use redis::AsyncCommands;
use shared::error::BunnyChessApiError;

use crate::{repositories::matchmaking_queue_repository::PlayerStatus, services::player_status_service::MatchMakingStatus};


#[async_trait::async_trait]
pub trait PlayerStatusRepository: Send + Sync {
    async fn get_player_status(
        &self,
        account_id: &str,
    ) -> Result<MatchMakingStatus, BunnyChessApiError>;
}

// #[derive(Clone)]
pub struct PlayerStatusRepositoryService {
    client: redis::Client,
}

impl PlayerStatusRepositoryService {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client
        }
    }

    pub fn get_account_status_key(account_id: &str) -> String {
        format!("matchmaking:account:{}:status", account_id)
    }

    fn map_redis_data_to_status(&self, data: HashMap<String, String>) -> MatchMakingStatus {
        MatchMakingStatus {
            status: data.get("status")
                .and_then(|s| s.parse().ok())
                .unwrap_or(PlayerStatus::Undefined),

            game_type: data.get("game_type")
                .and_then(|gt| gt.parse().ok()),

            ranked: data.get("ranked")
                .and_then(|r| r.parse().ok()),

            game_id: data.get("game_id")
                .map(|id| id.to_string()),
        }
    }
}

#[async_trait::async_trait]
impl PlayerStatusRepository for PlayerStatusRepositoryService {
    async fn get_player_status(
        &self,
        account_id: &str,
    ) -> Result<MatchMakingStatus, BunnyChessApiError> {
        let account_status_key = Self::get_account_status_key(account_id);

        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let data: HashMap<String, String> = conn.hgetall(&account_status_key).await
            .map_err(|e| BunnyChessApiError::RedisError(e))?;

        if data.is_empty() {
            Ok(MatchMakingStatus {
                status: PlayerStatus::Undefined,
                game_type: None,
                ranked: None,
                game_id: None,
            })
        } else {
            Ok(self.map_redis_data_to_status(data))
        }
    }
}
