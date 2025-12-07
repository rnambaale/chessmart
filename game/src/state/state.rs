
use std::sync::Arc;

use shared::error::ChessmartApiError;

use crate::{client::{nats::{NatsDB, NatsJetstreamContext}, redis::{RedisClient, RedisDB}}, config::{ApiConfig, NatsConfig, RedisConfig, ServerConfig, TracingConfig}};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ApiConfig>,
    pub redis: Arc<RedisClient>,
    pub jetstream: Arc<NatsJetstreamContext>,
}

impl AppState
{
    pub fn new(config: ApiConfig, redis: RedisClient, jetstream: NatsJetstreamContext) -> Self {
        let config = Arc::new(config);
        let redis = Arc::new(redis);
        let jetstream = Arc::new(jetstream);

        Self {
            config,
            redis,
            jetstream,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppStateBuilder {
    server_config: Option<ServerConfig>,
    tracing_config: Option<TracingConfig>,
    redis_config: Option<RedisConfig>,
    nats_config: Option<NatsConfig>,
}

impl AppStateBuilder {
    pub fn new() -> Self {
        AppStateBuilder {
            server_config: None,
            tracing_config: None,
            redis_config: None,
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

    pub async fn build(self) -> Result<AppState, ChessmartApiError> {
        let redis_config = self.redis_config.expect("redis-config not set");
        let redis = RedisDB::new(&redis_config).await?;

        let nats_config = self.nats_config.expect("nats-config not set");
        let jetstream = NatsDB::new(&nats_config).await?;

        Ok(AppState::new(
            ApiConfig::new(
                self.server_config.unwrap_or_default(),
                self.tracing_config,
                redis_config,
                nats_config,
            ),
            redis,
            jetstream
        ))
    }
}
