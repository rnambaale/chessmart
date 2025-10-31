use std::{collections::HashMap, sync::Arc};

use shared::{QueueSize, error::BunnyChessApiError, primitives::GameType};

use crate::{repositories::matchmaking_queue_repository::{MatchmakingQueueContract, PlayerStatus, QueueConfig, QueueType}, services::{player_status_service::{MatchMakingStatus, PlayerStatusServiceContract}, ranking_service::RankingServiceContract}};

pub struct AddToQueue {
    pub account_id: String,
    pub game_type: GameType,
    pub ranked: bool,
}

// Repository service (similar to NestJS)
pub struct MatchmakingQueueService {
    matchmaking_queue_repository: Arc<dyn MatchmakingQueueContract>,
    player_status_service: Arc<dyn PlayerStatusServiceContract>,
    ranking_service: Arc<dyn RankingServiceContract>,
}

impl MatchmakingQueueService {
    pub fn new(
        matchmaking_queue_repository: Arc<dyn MatchmakingQueueContract>,
        player_status_service: Arc<dyn PlayerStatusServiceContract>,
        ranking_service: Arc<dyn RankingServiceContract>
    ) -> Self {
        Self { matchmaking_queue_repository, player_status_service, ranking_service }
    }

    const RANKED_CONFIG: QueueConfig = QueueConfig {
        base_mmr_range: 50,
        mmr_range_increase_per_second: 5,
        max_mmr_delta: 400
    };

    const NORMAL_CONFIG: QueueConfig = QueueConfig {
        base_mmr_range: 100,
        mmr_range_increase_per_second: 10,
        max_mmr_delta: 600
    };

    pub async fn match_players(
        &self,
        game_type: &GameType,
        ranked: bool,
    ) -> redis::RedisResult<Vec<String>> {

        let queue_config = match  ranked {
            true => Self::RANKED_CONFIG,
            false => Self::NORMAL_CONFIG
        };

        self.matchmaking_queue_repository.match_players_in_queue(
            game_type,
            ranked,
            &queue_config,
        ).await
    }

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


        tracing::debug!(
            "Player {} added to {} {} queue",
            &account_id.as_str(),
            if ranked { "ranked" } else { "normal" },
            game_type.to_str()
        );

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

        tracing::debug!(
           "Player {} removed from {} {} queue",
           account_id,
           if ranked { "ranked" } else { "normal" },
           game_type.to_str()
        );

        Ok(())
    }

    pub async fn get_queue_sizes(&self) -> Result<HashMap<String, QueueSize>, BunnyChessApiError> {
        let game_types = vec![
            GameType::Rapid10_0,
            GameType::Blitz5_3,
            GameType::Blitz5_0,
            GameType::Blitz3_2,
            GameType::Blitz3_0,
            GameType::Bullet1_0,
        ];

        let queue_types: Vec<QueueType> = game_types.into_iter()
            .flat_map(|game_type| {
                [true, false]
                    .into_iter()
                    .map(move |ranked| QueueType {
                        game_type,
                        ranked,
                    })
            })
            .collect();

        Ok(self.matchmaking_queue_repository.get_queue_sizes(queue_types).await?)
    }
}
