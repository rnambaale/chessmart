use crate::config::RedisConfig;

#[derive(Clone)]
pub struct RedisDB { }

pub type RedisClient = redis::Client;

impl RedisDB {
    pub async fn new(config: &RedisConfig) -> Result<RedisClient, redis::RedisError> {
        Ok(redis::Client::open(config.get_url())?)
    }
}
