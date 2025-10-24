use std::{collections::HashMap, time::Duration};

use tracing::info;

use crate::config::RedisConfig;

#[derive(Clone)]
pub struct RedisDB { }

pub type RedisClient = redis::Client;

impl RedisDB {
    pub async fn new(config: &RedisConfig) -> Result<RedisClient, redis::RedisError> {
        Ok(redis::Client::open(config.get_url())?)
    }
}

pub trait RedisClientExt {
    fn ping(&self) -> impl std::future::Future<Output = Result<Option<String>, redis::RedisError>>;

    fn set(
        &self,
        key: &str,
        value: &str,
        expire: Duration,
    ) -> impl std::future::Future<Output = Result<(), redis::RedisError>>;

    fn hset(
        &self,
        key: &str,
        value: &[(&str, &str)],
        expire: Duration,
    ) -> impl std::future::Future<Output = Result<(), redis::RedisError>>;

    fn exist(&self, key: &str) -> impl std::future::Future<Output = Result<bool, redis::RedisError>>;

    fn get(&self, key: &str) -> impl std::future::Future<Output = Result<Option<String>, redis::RedisError>>;

    fn hgetall(&self, key: &str) -> impl std::future::Future<Output = Result<Option<HashMap<String, String>>, redis::RedisError>>;

    fn del(&self, key: &str) -> impl std::future::Future<Output = Result<bool, redis::RedisError>>;

    fn ttl(&self, key: &str) -> impl std::future::Future<Output = Result<i64, redis::RedisError>>;
}

impl RedisClientExt for redis::Client {
    async fn ping(&self) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("PING").query_async(&mut conn).await?;
        info!("ping redis server");
        Ok(value)
    }

    async fn set(&self, key: &str, value: &str, expire: Duration) -> Result<(), redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let msg: String = redis::cmd("SET")
            .arg(&[key, value])
            .query_async(&mut conn)
            .await?;
        info!("set key redis: {msg}");

        let msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await?;
        info!("set expire time redis: {msg}");
        Ok(())
    }

    async fn hset(&self, key: &str, values: &[(&str, &str)], expire: Duration) -> Result<(), redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let mut args = vec![key];

        for (field, value) in values {
            args.push(field);
            args.push(value);
        }

        let msg: String = redis::cmd("HSET")
            .arg(&args)
            .query_async(&mut conn)
            .await?;
        info!("set key redis: {msg}");

        let msg: i32 = redis::cmd("EXPIRE")
            .arg(&[key, &expire.as_secs().to_string()])
            .query_async(&mut conn)
            .await?;
        info!("set expire time redis: {msg}");
        Ok(())
    }

    async fn exist(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: bool = redis::cmd("EXISTS").arg(key).query_async(&mut conn).await?;
        info!("check key exists: {key}");
        Ok(value)
    }

    async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: Option<String> = redis::cmd("GET").arg(key).query_async(&mut conn).await?;
        info!("get value: {key}");
        Ok(value)
    }

    async fn hgetall(&self, key: &str) -> Result<Option<HashMap<String, String>>, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;

        let exists = self.exist(key).await?;
        if !exists {
            return Ok(None);
        }

        let value: HashMap<String, String> = redis::cmd("HGETALL").arg(key).query_async(&mut conn).await?;
        info!("get value: {key}");
        Ok(Some(value))
    }

    async fn del(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i32 = redis::cmd("DEL").arg(key).query_async(&mut conn).await?;
        info!("delete value: {key}");
        Ok(value == 1)
    }

    async fn ttl(&self, key: &str) -> Result<i64, redis::RedisError> {
        let mut conn = self.get_multiplexed_async_connection().await?;
        let value: i64 = redis::cmd("TTL").arg(key).query_async(&mut conn).await?;
        info!("get TTL value: {key}");
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fake::{Fake, Faker};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_ping_redis_server() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();
        let resp = client.ping().await.unwrap();
        let pong = "PONG";
        assert!(matches!(resp, Some(p) if p == pong));
    }

    #[tokio::test]
    async fn test_set_key_redis() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();
        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();

        client
            .set(&key, &value, Duration::from_secs(5))
            .await
            .unwrap();
        let resp = client.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));

        let resp = client.ttl(&key).await.unwrap();
        assert!(resp > 0);
    }

    #[tokio::test]
    async fn test_exist_key_redis() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();

        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        client
            .set(&key, &value, Duration::from_secs(4))
            .await
            .unwrap();
        let resp = client.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = client.exist(&key).await.unwrap();
        assert!(resp);
        let key: String = Faker.fake();
        let resp = client.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_del_key_redis() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();

        let key: String = Faker.fake();
        let value = Uuid::new_v4().to_string();
        client
            .set(&key, &value, Duration::from_secs(4))
            .await
            .unwrap();
        let resp = client.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = client.exist(&key).await.unwrap();
        assert!(resp);
        client.del(&key).await.unwrap();
        let resp = client.exist(&key).await.unwrap();
        assert!(!resp);
    }

    #[tokio::test]
    async fn test_key_ttl_redis() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();

        let key: String = Faker.fake();
        let ttl = 4;
        let value = Uuid::new_v4().to_string();
        client
            .set(&key, &value, Duration::from_secs(ttl))
            .await
            .unwrap();
        let resp = client.get(&key).await.unwrap();
        assert!(matches!(resp, Some(v) if v == value));
        let resp = client.ttl(&key).await.unwrap();
        assert!(resp <= ttl as i64 && resp > 0);
        client.del(&key).await.unwrap();
        let resp = client.ttl(&key).await.unwrap();
        assert!(resp < 0);
    }

    #[tokio::test]
    async fn test_hset_key_redis() {
        let client = RedisDB::new(&RedisConfig::default()).await.unwrap();
        let key: String = Faker.fake();
        let field1: String = Faker.fake();
        let value1 = Uuid::new_v4().to_string();
        let field2: String = Faker.fake();
        let value2 = Uuid::new_v4().to_string();

        let hash_values = &[
            (field1.as_str(), value1.as_str()),
            (field2.as_str(), value2.as_str())
        ];

        client
            .hset(&key, hash_values, Duration::from_secs(5))
            .await
            .unwrap();
        let resp = client.hgetall(&key).await.unwrap();

        let expected: HashMap<String, String> = hash_values
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        assert!(matches!(resp, Some(v) if v == expected));
    }
}
