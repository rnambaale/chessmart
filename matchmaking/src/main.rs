use std::{str::FromStr, sync::Arc};

use shared::{AcceptPendingGameRequest, AcceptPendingGameResponse, AddToQueueRequestPb, AddToQueueResponse, GetAccountStatusRequest, GetAccountStatusResponse, GetQueueSizesRequest, GetQueueSizesResponse, MatchmakerService, MatchmakerServiceServer, RemoveFromQueueRequest, RemoveFromQueueResponse, primitives::GameType};
use tonic::transport::Server;

use crate::{config::ApiConfig, repositories::{matchmaking_queue_repository::RedisMatchmakingQueue, player_status_repository::PlayerStatusRepositoryService}, server::state::{AppState, AppStateBuilder}, services::{matchmaking_queue_service::MatchmakingQueueRepositoryService, player_status_service::PlayerStatusServiceImpl, ranking::MyRankingService}};

pub mod services;
mod config;
mod redis;
mod database;
mod server;
mod repositories;

// #[derive(Debug, Default)]
pub struct MyMatchmakerService {
    // redis: Arc<RedisDB>,
    // state: Arc<AppState>,
    matchmaking_queue_repository: MatchmakingQueueRepositoryService,
}

impl MyMatchmakerService {
    pub fn new(
        // state: Arc<AppState>,
        matchmaking_queue_repository: MatchmakingQueueRepositoryService,
    ) -> Self {
        Self {
            // state,
            matchmaking_queue_repository
        }
    }
}


#[tonic::async_trait]
impl MatchmakerService for MyMatchmakerService {
    async fn add_to_queue(
        &self,
        request: tonic::Request<AddToQueueRequestPb>,
    ) -> Result<tonic::Response<AddToQueueResponse>, tonic::Status> {
        let AddToQueueRequestPb {
            account_id,
            ranked,
            game_type
        } = request.into_inner();

        let ranking = MyRankingService::get_or_create_ranking(&account_id).await?;

        let mmr = match ranked {
            true => ranking.ranked_mmr,
            false => ranking.normal_mmr,
        };

        self.matchmaking_queue_repository.add_player_to_queue(
            &account_id,
            mmr,
            &GameType::from_str(&game_type)?,
            ranked
        ).await.unwrap();

        Ok(tonic::Response::new(AddToQueueResponse{}))
        /*

        const ranking = await this.rankingService.getOrCreateRanking(accountId);
        await this.matchmakingQueueRepository.addPlayerToQueue({
        accountId,
        mmr: ranked ? ranking.rankedMmr : ranking.normalMmr,
        gameType,
        ranked,
        });
        this.logger.debug(
        `Player ${accountId} added to ${ranked ? 'ranked' : 'normal'} ${gameType} queue`,
        );
         */
    }

    async fn accept_pending_game(
        &self,
        _request: tonic::Request<AcceptPendingGameRequest>,
    ) -> Result<tonic::Response<AcceptPendingGameResponse>, tonic::Status> {
        todo!()
    }

    async fn remove_from_queue(
        &self,
        request: tonic::Request<RemoveFromQueueRequest>,
    ) -> Result<tonic::Response<RemoveFromQueueResponse>, tonic::Status> {
        let RemoveFromQueueRequest {
            account_id,
        } = request.into_inner();

        self.matchmaking_queue_repository.remove_player_from_queue(&account_id).await?;

        Ok(tonic::Response::new(RemoveFromQueueResponse{}))
    }

    async fn get_account_status(
        &self,
        _request: tonic::Request<GetAccountStatusRequest>,
    ) -> Result<tonic::Response<GetAccountStatusResponse>, tonic::Status> {
        todo!()
    }

    async fn get_queue_sizes(
        &self,
        _request: tonic::Request<GetQueueSizesRequest>,
    ) -> Result<tonic::Response<GetQueueSizesResponse>, tonic::Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://localhost:6379".to_string());
    println!("Connecting to Redis at: {}", redis_url);

    let ApiConfig {
        server,
        database,
        tracing,
        redis,
        token_secret,
    } = ApiConfig::read_config_with_defaults();

    let state: AppState = AppStateBuilder::new()
        .with_server(Some(server))
        .with_db(Some(database))
        .with_tracing(tracing)
        .with_redis(Some(redis))
        .with_token_secret(Some(token_secret))
        .build()
        .await?;

    let addr = "[::1]:50051".parse()?;

    let player_status_repos = PlayerStatusRepositoryService::new();

    let matchmaking_queue_repository = MatchmakingQueueRepositoryService::new(
        Arc::new(RedisMatchmakingQueue::new(state.redis.clone())),
        Arc::new(PlayerStatusServiceImpl::new(player_status_repos))
    );

    let matchmaker_service = MyMatchmakerService::new(
        // Arc::new(state),
        matchmaking_queue_repository,
    );

    println!("MatchmakerService gRPC server running on {}", addr);

    Server::builder()
        .add_service(MatchmakerServiceServer::new(matchmaker_service))
        // .add_service(RankingServiceServer::new(ranking_service))
        // .add_service(RankingServiceServer::new(MyRankingService))
        .serve(addr)
        .await?;

    Ok(())
}
