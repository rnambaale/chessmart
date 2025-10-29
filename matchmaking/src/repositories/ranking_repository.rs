
use std::str::FromStr;

use shared::error::BunnyChessApiError;
use sqlx::types::Uuid;

use crate::{database::{self, Database, postgres::PostgresDB}, services::ranking::Ranking};

#[async_trait::async_trait]
pub trait RankingRepository: Send + Sync {
    async fn find_ranking(
        &self,
        account_id: &str,
    ) -> Result<Ranking, BunnyChessApiError>;

    async fn insert_ranking(
        &self,
        ranking: &Ranking,
    ) -> Result<(), BunnyChessApiError>;
}

// pub struct RankingInsert {
//     pub account_id: String,
//     pub ranked_mmr: u16,
//     pub normal_mmr: u16,
// }

// pub struct RankingUpdate {
//     pub game_id: String,
//     pub game_type: String,
//     pub ranked:bool,
//     pub account_id: String,
//     pub mmr_change: u16,
// }

pub struct RankingRepositoryService {
    client: PostgresDB,
}

impl RankingRepositoryService
{
    pub fn new(client: PostgresDB) -> Self {
        Self { client }
    }

    // pub async fn update_rankings(updates: &[RankingUpdate]) {
    //     todo!()
    // }
}

#[async_trait::async_trait]
impl RankingRepository for RankingRepositoryService {
    async fn find_ranking(
        &self,
        account_id: &str
    ) -> Result<Ranking, BunnyChessApiError> {
        let mut tx = self.client.begin_tx()
            .await
            .map_err(|e| BunnyChessApiError::Db(e))?;

        let row = sqlx::query!(
            r#"
            SELECT id, account_id, ranked_mmr, normal_mmr, created_at
            FROM rankings
            WHERE account_id = $1
            "#,
            account_id
        )
        //.fetch_optional(&mut *tx)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| BunnyChessApiError::Db(e))?;

        tx.commit()
            .await
            .map_err(|e| BunnyChessApiError::Db(e))?;

        // match row {
        //     Some(record) => Ok(Ranking {
        //         id: record.id.to_string(),
        //         account_id: record.account_id,
        //         ranked_mmr: record.ranked_mmr as u16,
        //         normal_mmr: record.normal_mmr as u16,
        //         created_at: record.created_at.unwrap(),
        //     }),
        //     None => Err(BunnyChessApiError::RankingNotFound(account_id.to_string())),
        // }
        Ok(
            Ranking {
                id: row.id.to_string(),
                account_id: row.account_id,
                ranked_mmr: row.ranked_mmr as u16,
                normal_mmr: row.normal_mmr as u16,
                created_at: row.created_at.unwrap(),
            }
        )
    }

    async fn insert_ranking(
        &self,
        ranking: &Ranking,
    ) -> Result<(), BunnyChessApiError> {
        let mut tx = self.client.begin_tx()
            .await?;

        sqlx::query!(
            "INSERT INTO rankings (id, account_id, ranked_mmr, normal_mmr, created_at) VALUES ($1, $2, $3, $4, $5)",
            Uuid::from_str(ranking.id.as_str())?,
            ranking.account_id,
            ranking.ranked_mmr as i32,
            ranking.normal_mmr as i32,
            ranking.created_at
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
