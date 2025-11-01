use shared::error::BunnyChessApiError;
use tracing::info;
use uuid::Uuid;

use crate::{redis::redis::RedisClient, services::redis::SessionKey, utils::claim::UserClaims};

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

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> Result<Uuid, BunnyChessApiError> {
  let session_key = SessionKey {
    user_id: claims.uid,
  };
  let session_id = crate::services::redis::get(redis, &session_key)
    .await?
    .ok_or_else(|| {
      BunnyChessApiError::SessionNotFoundError("Session not found".into())
    })?;
  if claims.sid != session_id {
    info!("Session id invalid so deleting it: {session_key:?}.");
    crate::services::redis::del(redis, &session_key).await?;
    return Err(BunnyChessApiError::InvalidSessionError(
      "Session is Invalid".to_string(),
    ));
  }
  Ok(claims.uid)
}
