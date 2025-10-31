
use shared::error::BunnyChessApiError;

use crate::{config::{ApiConfig, DatabaseConfig, RedisConfig, ServerConfig, TracingConfig}, database::{Database, postgres::PostgresDB}, redis::redis::{RedisClient, RedisDB}};

// #[derive(Clone)]
pub struct AppState<DB: Database = PostgresDB> {
    pub db: DB,
    pub config: ApiConfig,
    pub redis: RedisClient,
}

impl<DB> AppState<DB>
where
    DB: Database,
{
    pub fn new(db: DB, config: ApiConfig, redis: RedisClient) -> Self {
        Self {
            db,
            config,
            redis,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    db_config: Option<DatabaseConfig>,
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            db_config: None,
            server_config: None,
            tracing_config: None,
            redis_config: None,
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

    pub async fn build(self) -> Result<AppState<PostgresDB>, BunnyChessApiError> {
        let db_config = self.db_config.expect("db-config not set");
        let db = PostgresDB::new(&db_config).await?;
        db.migrate().await;

        let redis_config = self.redis_config.expect("redis-config not set");
        let redis = RedisDB::new(&redis_config).await?;

        Ok(AppState::new(
            db,
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                db_config,
                self.tracing_config,
                redis_config,
            ),
            redis
        ))
    }
}
