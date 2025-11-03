use shared::generated::{account_service::account_service_client::AccountServiceClient, matchmaker_service::matchmaker_service_client::MatchmakerServiceClient, ranking_service::ranking_service_client::RankingServiceClient};
use tonic::transport::Channel;

use crate::{config::{ApiConfig, DatabaseConfig, RedisConfig, ServerConfig, TokenSecretConfig, TracingConfig}, database::{postgres::PostgresDB, Database}, error::BunnyChessApiError, redis::redis::{RedisClient, RedisDB}};

type AccountGrpcClient = AccountServiceClient<Channel>;
pub type MatchmakingGrpcClient = MatchmakerServiceClient<Channel>;
type RankingGrpcClient = RankingServiceClient<Channel>;

#[derive(Clone)]
pub struct AppState<DB: Database = PostgresDB> {
    pub db: DB,
    pub config: ApiConfig,
    pub redis: RedisClient,
    pub account_client: AccountGrpcClient,
    pub matchmaking_client: MatchmakingGrpcClient,
    pub ranking_client: RankingGrpcClient,
}

impl<DB> AppState<DB>
where
    DB: Database,
{
    pub fn new(
        db: DB,
        config: ApiConfig,
        redis: RedisClient,
        account_client: AccountGrpcClient,
        matchmaking_client: MatchmakingGrpcClient,
        ranking_client: RankingGrpcClient,
    ) -> Self {
        Self {
            db,
            config,
            redis,
            account_client,
            matchmaking_client,
            ranking_client,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    db_config: Option<DatabaseConfig>,
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>,
    token_secret_config: Option<TokenSecretConfig>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            db_config: None,
            server_config: None,
            tracing_config: None,
            redis_config: None,
            token_secret_config: None,
        }
    }

    pub fn with_db(mut self, db_config: Option<DatabaseConfig>) -> Self {
        self.db_config = db_config;
        self
    }

    pub fn with_redis(mut self, redis_config: Option<RedisConfig>) -> Self {
        self.redis_config = redis_config;
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

    pub async fn build(self) -> Result<AppState<PostgresDB>, BunnyChessApiError> {
        let db_config = self.db_config.expect("db-config not set");
        let db = PostgresDB::new(&db_config).await?;
        db.migrate().await;

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

        Ok(AppState::new(
            db,
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                db_config,
                self.tracing_config,
                redis_config,
                token_secret_config,
            ),
            redis,
            account_client,
            matchmaking_client,
            ranking_client,
        ))
    }
}
