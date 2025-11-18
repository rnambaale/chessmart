use shared::error::BunnyChessApiError;

use crate::{client::{database::{Database, PostgresDB}, nats::{NatsDB, NatsJetstreamContext}, redis::{RedisClient, RedisDB}}, config::{ApiConfig, DatabaseConfig, NatsConfig, RedisConfig, ServerConfig, TracingConfig}};

pub struct AppState<DB: Database = PostgresDB> {
    pub db: DB,
    pub config: ApiConfig,
    pub redis: RedisClient,
    pub jetstream: NatsJetstreamContext,
}

impl<DB> AppState<DB>
where
    DB: Database,
{
    pub fn new(db: DB, config: ApiConfig, redis: RedisClient, jetstream: NatsJetstreamContext) -> Self {
        Self {
            db,
            config,
            redis,
            jetstream,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    db_config: Option<DatabaseConfig>,
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>,
    nats_config: Option<NatsConfig>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            db_config: None,
            server_config: None,
            tracing_config: None,
            redis_config: None,
            nats_config: None,
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

    pub async fn build(self) -> Result<AppState<PostgresDB>, BunnyChessApiError> {
        let db_config = self.db_config.expect("db-config not set");
        let db = PostgresDB::new(&db_config).await?;
        db.migrate().await;

        let redis_config = self.redis_config.expect("redis-config not set");
        let redis = RedisDB::new(&redis_config).await?;

        let nats_config = self.nats_config.expect("nats-config not set");
        let jetstream = NatsDB::new(&nats_config).await?;

        Ok(AppState::new(
            db,
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                db_config,
                self.tracing_config,
                redis_config,
                nats_config,
            ),
            redis,
            jetstream,
        ))
    }
}
