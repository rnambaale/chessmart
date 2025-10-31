use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared::error::BunnyChessApiError;
use sqlx::types::Uuid;

use crate::{repositories::ranking_repository::RankingRepository};

 #[async_trait::async_trait]
pub trait RankingServiceContract: Send + Sync {
    async fn get_or_create_ranking(&self, account_id: &str) -> Result<Ranking, BunnyChessApiError>;
}

pub struct Ranking {
    pub id: String,
    pub account_id: String,
    pub ranked_mmr: u16,
    pub normal_mmr: u16,
    pub created_at: DateTime<Utc>,
}

pub struct MyRankingService {
    ranking_repository: Arc<dyn RankingRepository>,
}

impl MyRankingService {
    pub const STARTING_MMR: u16 = 1000;

    pub fn new(
        ranking_repository: Arc<dyn RankingRepository>,
    ) -> Self {
        Self { ranking_repository }
    }
}

#[async_trait::async_trait]
impl RankingServiceContract for MyRankingService {
    async fn get_or_create_ranking(&self, account_id: &str) -> Result<Ranking, BunnyChessApiError> {
        let record: Option<Ranking> = self.ranking_repository.find_ranking(account_id).await?;

        if let Some(ranking) = record {
            return Ok(ranking);
        }

        let ranking = Ranking {
            account_id: account_id.to_string(),
            id: Uuid::new_v4().to_string(),
            ranked_mmr: Self::STARTING_MMR,
            normal_mmr: Self::STARTING_MMR,
            created_at: Utc::now(),
        };

        self.ranking_repository.insert_ranking(&ranking).await?;

        Ok(ranking)
    }
}
