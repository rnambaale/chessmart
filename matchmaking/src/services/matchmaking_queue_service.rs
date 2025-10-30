use std::sync::Arc;

use shared::{error::BunnyChessApiError, primitives::GameType};

use crate::{repositories::matchmaking_queue_repository::{MatchmakingQueue, PlayerStatus}, services::{player_status_service::{MatchMakingStatus, PlayerStatusService}, ranking::RankingService}};

pub struct AddToQueue {
    pub account_id: String,
    pub game_type: GameType,
    pub ranked: bool,
}

// Repository service (similar to NestJS)
pub struct MatchmakingQueueService {
    matchmaking_queue_repository: Arc<dyn MatchmakingQueue>,
    player_status_service: Arc<dyn PlayerStatusService>,
    ranking_service: Arc<dyn RankingService>,
}

impl MatchmakingQueueService {
    pub fn new(
        matchmaking_queue_repository: Arc<dyn MatchmakingQueue>,
        player_status_service: Arc<dyn PlayerStatusService>,
        ranking_service: Arc<dyn RankingService>
    ) -> Self {
        Self { matchmaking_queue_repository, player_status_service, ranking_service }
    }

    // pub async fn match_players(
    //     &self,
    //     queue_key: &str,
    //     times_key: &str,
    //     base_mmr_range: i32,
    //     mmr_increase_per_second: f64,
    //     max_mmr_delta: i32,
    // ) -> RedisResult<Vec<String>> {
    //     self.redis.match_players(
    //         queue_key,
    //         times_key,
    //         base_mmr_range,
    //         mmr_increase_per_second,
    //         max_mmr_delta,
    //     ).await
    // }

    pub async fn add_player_to_queue(
        &self,
        payload: AddToQueue
    ) -> Result<(), BunnyChessApiError> {

        let AddToQueue {
            account_id,
            ranked,
            game_type,
        } = payload;

        let ranking = self.ranking_service.get_or_create_ranking(&account_id).await?;

        let mmr = match ranked {
            true => ranking.ranked_mmr,
            false => ranking.normal_mmr,
        };

        self.matchmaking_queue_repository.add_player_to_queue(
            account_id.as_str(),
            mmr,
            &game_type,
            ranked,
        ).await?;

        Ok(())
    }

    pub async fn remove_player_from_queue(
        &self,
        account_id: &str,
    ) -> Result<(), BunnyChessApiError> {
        let MatchMakingStatus {
            status,
            game_type,
            ranked,
            ..
        } = self.player_status_service.get_player_status(account_id).await?;

        if !(matches!(status, PlayerStatus::Searching) && game_type.is_some() && ranked.is_some()) {
            return Ok(());
        }

        let game_type = game_type.unwrap();
        let ranked = ranked.unwrap();

        self.matchmaking_queue_repository.remove_player_from_queue(
            account_id,
            game_type,
            ranked
        ).await?;

        Ok(())
    }
}
