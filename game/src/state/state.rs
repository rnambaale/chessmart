
use std::sync::Arc;

use shared::error::BunnyChessApiError;

use crate::{client::{redis::{RedisClient, RedisDB}}, config::{ApiConfig, RedisConfig, ServerConfig, TracingConfig}};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApiConfig>,
    pub redis: Arc<RedisClient>,
}

impl AppState
{
    pub fn new(config: ApiConfig, redis: RedisClient) -> Self {
        let config = Arc::new(config);
        let redis = Arc::new(redis);

        Self {
            config,
            redis,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            server_config: None,
            tracing_config: None,
            redis_config: None,
        }
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

    pub async fn build(self) -> Result<AppState, BunnyChessApiError> {
        let redis_config = self.redis_config.expect("redis-config not set");
        let redis = RedisDB::new(&redis_config).await?;

        Ok(AppState::new(
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                self.tracing_config,
                redis_config,
            ),
            redis
        ))
    }
}
