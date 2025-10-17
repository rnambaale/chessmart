use uuid::Uuid;

use crate::{error::BunnyChessApiError, redis::redis::RedisClient, services::redis::SessionKey};

pub async fn set(redis: &RedisClient, user_id: Uuid) -> Result<Uuid, BunnyChessApiError> {
  let (key, value) = generate(user_id);
  crate::services::redis::set(redis, (&key, &value)).await?;
  Ok(value)
}

pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
  let session_id = Uuid::new_v4();
  let key = SessionKey { user_id };
  (key, session_id)
}
