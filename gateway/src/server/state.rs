use shared::generated::{account_service::account_service_client::AccountServiceClient, game_service::game_service_client::GameServiceClient, matchmaker_service::matchmaker_service_client::MatchmakerServiceClient, ranking_service::ranking_service_client::RankingServiceClient};
use tonic::transport::Channel;

use crate::{client::nats::{NatsDB, NatsJetstreamContext}, config::{ApiConfig, NatsConfig, RedisConfig, ServerConfig, TokenSecretConfig, TracingConfig}, error::GatewayServiceError, client::redis::{RedisClient, RedisDB}};

type AccountGrpcClient = AccountServiceClient<Channel>;
pub type MatchmakingGrpcClient = MatchmakerServiceClient<Channel>;
type RankingGrpcClient = RankingServiceClient<Channel>;
pub type GameGrpcClient = GameServiceClient<Channel>;

#[derive(Clone)]
pub struct AppState {
    pub config: ApiConfig,
    pub redis: RedisClient,
    pub account_client: AccountGrpcClient,
    pub matchmaking_client: MatchmakingGrpcClient,
    pub ranking_client: RankingGrpcClient,
    pub game_client: GameGrpcClient,
    pub jetstream: NatsJetstreamContext,
}

impl AppState {
    pub fn new(
        config: ApiConfig,
        redis: RedisClient,
        account_client: AccountGrpcClient,
        matchmaking_client: MatchmakingGrpcClient,
        ranking_client: RankingGrpcClient,
        game_client: GameGrpcClient,
        jetstream: NatsJetstreamContext,
    ) -> Self {
        Self {
            config,
            redis,
            account_client,
            matchmaking_client,
            ranking_client,
            game_client,
            jetstream,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>,
    token_secret_config: Option<TokenSecretConfig>,
    nats_config: Option<NatsConfig>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            server_config: None,
            tracing_config: None,
            redis_config: None,
            token_secret_config: None,
            nats_config: None,
        }
    }

    pub fn with_redis(mut self, redis_config: Option<RedisConfig>) -> Self {
        self.redis_config = redis_config;
        self
    }

    pub fn with_nats(mut self, nats_config: Option<NatsConfig>) -> Self {
        self.nats_config = nats_config;
        self
    }

    pub fn with_server(mut self, server_config: Option<ServerConfig>) -> Self {
        self.server_config = server_config;
        self
    }

    pub fn with_tracing(mut self, tracing_config: Option<TracingConfig>) -> Self {
        self.tracing_config = tracing_config;
        self
    }

    pub fn with_token_secret(mut self, token_secret_config: Option<TokenSecretConfig>) -> Self {
        self.token_secret_config = token_secret_config;
        self
    }

    pub async fn build(self) -> Result<AppState, GatewayServiceError> {
        let redis_config = self.redis_config.expect("redis-config not set");
        let redis = RedisDB::new(&redis_config).await?;

        let token_secret_config = self.token_secret_config.expect("token-secret-config not set");

        // Initialize account gRPC client
        let account_channel = Channel::from_static("http://[::1]:50051")
            .connect()
            .await?;

        let account_client = AccountServiceClient::new(account_channel);

        // Initialize matchmaking gRPC client
        let matchmaking_channel = Channel::from_static("http://[::1]:50052")
            .connect()
            .await?;

        let matchmaking_client = MatchmakerServiceClient::new(matchmaking_channel.clone());
        let ranking_client = RankingServiceClient::new(matchmaking_channel.clone());

        let game_channel = Channel::from_static("http://[::1]:50053")
            .connect()
            .await?;
        let game_client = GameServiceClient::new(game_channel.clone());

        let nats_config = self.nats_config.expect("nats-config not set");
        let jetstream = NatsDB::new(&nats_config).await?;

        Ok(AppState::new(
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                self.tracing_config,
                redis_config,
                token_secret_config,
                nats_config,
            ),
            redis,
            account_client,
            matchmaking_client,
            ranking_client,
            game_client,
            jetstream,
        ))
    }
}
