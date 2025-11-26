use std::{fmt::{Debug, Display}, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{constants::EXPIRE_SESSION_CODE_SECS, error::GatewayServiceError, client::redis::{RedisClient, RedisClientExt}};

pub trait RedisKey: Debug + Display {
  type Value: Serialize + DeserializeOwned + Debug;
  const EXPIRE_TIME: Duration;
  fn expire(&self) -> Duration {
    Self::EXPIRE_TIME
  }
}

#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct SessionKey {
  pub user_id: Uuid,
}

impl RedisKey for SessionKey  {
    type Value = Uuid;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

impl Display for SessionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SESSION_KEY_{}", self.user_id)
    }
}

impl RedisKey for String  {
    type Value = String;
    const EXPIRE_TIME: Duration = EXPIRE_SESSION_CODE_SECS;
}

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> Result<(), GatewayServiceError>
where
  K: RedisKey,
{
  info!("Set value to redis key :{key:?} value :{value:?}");
  let value = serde_json::to_string(value)?;
  client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
  Ok(())
}

// pub async fn hset<K>(client: &RedisClient, key: &K, value: &PlayerStatusUpdate) -> Result<(), GatewayServiceError>
// where
//   K: RedisKey,
// {
//   // info!("Set value to redis key :{key:?} value :{value:?}");
//   // let value = serde_json::to_string(value)?;
//   let field_values: Vec<(&str, &str)> = vec![
//     ("status", &value.status.to_str()),
//     ("game_type", &value.game_type_to_str())
//   ];
//   // client.hset(&key.to_string(), &value, K::EXPIRE_TIME).await?;
//   Ok(())
// }

pub async fn get<K>(client: &RedisClient, key: &K) -> Result<Option<K::Value>, GatewayServiceError>
where
  K: RedisKey,
{
  info!("Get value from redis key :{key}");
  Ok(
    client
      .get(&key.to_string())
      .await?
      .map(|v| serde_json::from_str::<K::Value>(&v))
      .transpose()?,
  )
}

// pub async fn hgetall<K>(client: &RedisClient, key: &K) -> Result<Option<HashMap<String, String>>, GatewayServiceError>
// where
//   K: RedisKey,
// {
//   info!("Get hash values from redis key :{key}");

//   Ok(client.hgetall(&key.to_string()).await?)
// }

pub async fn del(client: &RedisClient, key: &impl RedisKey) -> Result<bool, redis::RedisError> {
  info!("Delete key in redis :{key:?}");
  client.del(&key.to_string()).await
}
