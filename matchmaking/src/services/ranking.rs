use chrono::{DateTime, Utc};
use shared::{GetAccountRankingRequest, GetAccountRankingResponse, RankingService};

/*
export interface Ranking {
  id?: string | undefined;
  accountId: string;
  rankedMmr: number;
  normalMmr: number;
  createdAt?: Date | undefined;
}
 */

pub struct Ranking {
    pub id: String,
    pub account_id: String,
    pub ranked_mmr: u16,
    pub normal_mmr: u16,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Default)]
pub struct MyRankingService;

#[tonic::async_trait]
impl RankingService for MyRankingService {
    async fn get_account_ranking(
        &self,
        _request: tonic::Request<GetAccountRankingRequest>,
    ) -> std::result::Result<
        tonic::Response<GetAccountRankingResponse>,
        tonic::Status,
    > {
        todo!()
    }
}

impl MyRankingService {
    pub async fn get_or_create_ranking(_account_id: &str) -> Result<Ranking, tonic::Status> {
        todo!()
    }
}
