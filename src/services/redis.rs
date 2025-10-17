use std::{fmt::{Debug, Display}, time::Duration};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{constants::EXPIRE_SESSION_CODE_SECS, error::BunnyChessApiError, redis::redis::{RedisClient, RedisClientExt}};

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

pub async fn set<K>(client: &RedisClient, (key, value): (&K, &K::Value)) -> Result<(), BunnyChessApiError>
where
  K: RedisKey,
{
  info!("Set value to redis key :{key:?} value :{value:?}");
  let value = serde_json::to_string(value)?;
  client.set(&key.to_string(), &value, K::EXPIRE_TIME).await?;
  Ok(())
}
