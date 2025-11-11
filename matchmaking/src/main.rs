use std::{str::FromStr, sync::Arc};

use opentelemetry_otlp::WithExportConfig;
use shared::{AcceptPendingGameRequest, AcceptPendingGameResponse, AddToQueueRequestPb, AddToQueueResponse, GetAccountRankingRequest, GetAccountRankingResponse, GetAccountStatusRequest, GetAccountStatusResponse, GetQueueSizesRequest, GetQueueSizesResponse, MatchmakerService, MatchmakerServiceServer, RankingService, RankingServiceServer, RemoveFromQueueRequest, RemoveFromQueueResponse, primitives::GameType};
use tonic::transport::Server;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::{ApiConfig, TracingConfig}, repositories::{matchmaking_queue_repository::RedisMatchmakingQueue, player_status_repository::PlayerStatusRepositoryService, ranking_repository::RankingRepositoryService}, state::state::{AppState, AppStateBuilder}, services::{matchmaking_queue_service::{AddToQueue, MatchmakingQueueService}, player_status_service::{MatchMakingStatus, PlayerStatusService, PlayerStatusServiceContract}, ranking_service::{MyRankingService, Ranking, RankingServiceContract}}};

pub mod services;
mod config;
mod state;
mod repositories;
mod client;

pub struct MatchmakerGatewayService {
    matchmaking_queue_service: MatchmakingQueueService,
    player_status_service: PlayerStatusService,
}

impl MatchmakerGatewayService {
    pub fn new(
        matchmaking_queue_service: MatchmakingQueueService,
        player_status_service: PlayerStatusService,
    ) -> Self {
        Self {
            matchmaking_queue_service,
            player_status_service,
        }
    }
}


#[tonic::async_trait]
impl MatchmakerService for MatchmakerGatewayService {
    async fn add_to_queue(
        &self,
        request: tonic::Request<AddToQueueRequestPb>,
    ) -> Result<tonic::Response<AddToQueueResponse>, tonic::Status> {
        let AddToQueueRequestPb {
            account_id,
            ranked,
            game_type
        } = request.into_inner();

        let game_type = GameType::from_str(&game_type)?;

        self.matchmaking_queue_service.add_player_to_queue(
            AddToQueue {
                account_id,
                game_type,
                ranked
            }
        ).await.unwrap();

        Ok(tonic::Response::new(AddToQueueResponse{}))
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

        self.matchmaking_queue_service.remove_player_from_queue(&account_id).await?;

        Ok(tonic::Response::new(RemoveFromQueueResponse{}))
    }

    async fn get_account_status(
        &self,
        request: tonic::Request<GetAccountStatusRequest>,
    ) -> Result<tonic::Response<GetAccountStatusResponse>, tonic::Status> {
        let GetAccountStatusRequest {
            account_id,
        } = request.into_inner();

        let MatchMakingStatus {
            status,
            game_type,
            game_id,
            ranked,
        } = self.player_status_service.get_player_status(&account_id).await?;

        let game_type = match game_type {
            Some(game_t) => Some(game_t.to_str().into()),
            None => None
        };

        Ok(
            tonic::Response::new(GetAccountStatusResponse{
                status: status.as_str().into(),
                game_type,
                game_id,
                ranked
            })
        )
    }

    async fn get_queue_sizes(
        &self,
        _request: tonic::Request<GetQueueSizesRequest>,
    ) -> Result<tonic::Response<GetQueueSizesResponse>, tonic::Status> {
        Ok(tonic::Response::new(
            GetQueueSizesResponse {
                queue_sizes: self.matchmaking_queue_service.get_queue_sizes().await?
            }
        ))
    }
}


pub struct RankingGatewayService {
    ranking_service: Arc<dyn RankingServiceContract>,
}

impl RankingGatewayService {
    pub fn new(
        ranking_service: Arc<dyn RankingServiceContract>,
    ) -> Self {
        Self {
            ranking_service,
        }
    }
}

#[tonic::async_trait]
impl RankingService for RankingGatewayService {
    async fn get_account_ranking(
        &self,
        request: tonic::Request<GetAccountRankingRequest>,
    ) -> std::result::Result<
        tonic::Response<GetAccountRankingResponse>,
        tonic::Status,
    > {
        let GetAccountRankingRequest { account_id } = request.into_inner();
        let ranking: Ranking = self.ranking_service.get_or_create_ranking(&account_id).await?;

        Ok(tonic::Response::new(
            GetAccountRankingResponse {
                ranked_mmr: ranking.ranked_mmr as f32,
                normal_mmr: ranking.normal_mmr as f32
            }
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // match dotenvy::dotenv() {
    //     Ok(path) => println!(".env read successfully from {}", path.display()),
    //     Err(e) => panic!("Could not load .env file: {e}"),
    // };

    let ApiConfig {
        server,
        database,
        tracing,
        redis,
    } = ApiConfig::read_config_with_defaults();

    init_tracing(tracing.clone())?;

    let state: AppState = AppStateBuilder::new()
        .with_server(Some(server))
        .with_db(Some(database))
        .with_tracing(tracing)
        .with_redis(Some(redis))
        .build()
        .await?;

    // let addr = server.host_port.clone();
    // let addr = "[::1]:50052".parse()?;
    let addr = state.config.server.host_port;

    let player_status_repository = PlayerStatusRepositoryService::new(state.redis.clone());

    let matchmaking_queue_service = MatchmakingQueueService::new(
        Arc::new(RedisMatchmakingQueue::new(state.redis.clone())),
        Arc::new(
            PlayerStatusService::new(Arc::new(player_status_repository))
        ),
        Arc::new(MyRankingService::new(
            Arc::new(
                RankingRepositoryService::new(state.db.clone())
            )
        ))
    );

    let player_status_service = PlayerStatusService::new(
        Arc::new(
            PlayerStatusRepositoryService::new(state.redis.clone())
        )
    );

    let matchmaker_gateway_service = MatchmakerGatewayService::new(
        // Arc::new(state),
        matchmaking_queue_service,
        player_status_service,
    );

    let ranking_gateway_service = RankingGatewayService::new(
        Arc::new(
            MyRankingService::new(
                Arc::new(RankingRepositoryService::new(state.db.clone()))
            )
        )
    );

    println!("MatchmakerService gRPC server running on {}", addr);

    Server::builder()
        .add_service(MatchmakerServiceServer::new(matchmaker_gateway_service))
        .add_service(RankingServiceServer::new(ranking_gateway_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn init_tracing(tr: Option<TracingConfig>) -> anyhow::Result<()> {
    let otlp_tracer = if tr.is_some() {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter().http().with_endpoint(
                    tr.unwrap_or_default()
                        .endpoint
                        .expect("No endpoint for tracing found"),
                ),
            )
            .with_trace_config(
                opentelemetry_sdk::trace::config()
                    .with_sampler(opentelemetry_sdk::trace::Sampler::AlwaysOn)
                    .with_resource(opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                        "service.name",
                        "bunny-chess matchmaker",
                    )])),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)?;
        Some(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(otlp_tracer)
        .try_init()?;
    Ok(())
}
