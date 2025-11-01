use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RegisterResponseDto {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponseDto {
  pub jwt: String,
  pub jwt_expires_in: DateTime<Utc>,
  pub jwt_refresh: String,
  pub jwt_refresh_expires_in: DateTime<Utc>,
}

impl LoginResponseDto {
  pub fn new(access_token: String, refresh_token: String, expire_in: DateTime<Utc>, jwt_refresh_expires_in: DateTime<Utc>) -> Self {
    Self {
      jwt: access_token,
      jwt_refresh: refresh_token,
      jwt_expires_in: expire_in,
      jwt_refresh_expires_in
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponseDto {
  pub message: String,
}

impl MessageResponseDto {
  pub fn new<S: Into<String>>(message: S) -> Self {
    Self {
      message: message.into(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MeResponseDto {
  pub id: String,
  pub email: String,
  pub username: String,
  pub is_admin: bool,
  pub created_at: DateTime<Utc>,
  pub last_login_at: Option<DateTime<Utc>>,
  pub status: String,
  pub game_type: Option<String>,
  pub ranked: Option<bool>,
  pub game_id: Option<String>,
  pub mmr: u64,
}
