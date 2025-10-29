// use std::sync::Arc;

// use shared::error::BunnyChessApiError;

// use crate::services::ranking::Ranking;

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

// pub struct RankingRepositoryService {
//     // database: Arc<dyn MatchmakingQueue>,
// }

// impl RankingRepositoryService {
//     pub fn new(
//         // redis: Arc<dyn MatchmakingQueue>,
//     ) -> Self {
//         Self { }
//     }

//     pub async fn insert_ranking(ranking: &RankingInsert) {
//         todo!()
//     }

//     pub async fn update_rankings(updates: &[RankingUpdate]) {
//         todo!()
//     }

//     pub async fn find_ranking(account_id: &str) -> Result<Option<Ranking>, BunnyChessApiError> {
//         todo!()
//     }
// }
