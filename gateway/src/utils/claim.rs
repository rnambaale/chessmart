use std::{sync::LazyLock, time::Duration};

use axum::{extract::FromRequestParts, http::request::Parts};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};

use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use async_trait::async_trait;

use crate::{constants::ACCESS_TOKEN_DECODE_KEY, error::BunnyChessApiError, server::state::AppState};

pub static DECODE_HEADER: LazyLock<Validation> =
  LazyLock::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, ToSchema)]
pub struct UserClaims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // user id
    pub uid: Uuid,
    // session id
    pub sid: Uuid,
}

impl UserClaims {
    pub fn new(duration: Duration, user_id: Uuid, session_id: Uuid) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: user_id,
            sid: session_id,
        }
    }

    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<UserClaims>(token, key, &DECODE_HEADER)
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key)
    }
}

#[async_trait]
impl FromRequestParts<AppState> for UserClaims
{
  type Rejection = BunnyChessApiError;

  async fn from_request_parts(
    parts: &mut Parts,
    state: &AppState,
  ) -> Result<Self, Self::Rejection> {
    // let TypedHeader(Authorization(bearer)) = parts
    //   .extract::<TypedHeader<Authorization<Bearer>>>()
    //   .await?;
    // let user_claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)?.claims;

    let auth_header = parts.headers
        .get("Authorization")
        .ok_or_else(|| BunnyChessApiError::UnauthorizedError("Missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| BunnyChessApiError::UnauthorizedError("Invalid Authorization header".to_string()))?;
    if !auth_header.starts_with("Bearer ") {
        return Err(BunnyChessApiError::UnauthorizedError("Invalid Authorization format".to_string()));
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();

    let user_claims = UserClaims::decode(token, &ACCESS_TOKEN_DECODE_KEY)?.claims;
    crate::services::session::check(&state.redis, &user_claims).await?;
    Ok(user_claims)
  }
}

pub trait UserClaimsRequest {
  fn get_user_id(&self) -> Result<Uuid, BunnyChessApiError>;
  fn get_user_claims(&self) -> Result<UserClaims, BunnyChessApiError>;
}

impl UserClaimsRequest for axum::extract::Request {
  fn get_user_id(&self) -> Result<Uuid, BunnyChessApiError> {
    self
      .extensions()
      .get::<UserClaims>()
      .map(|u| u.uid)
      .ok_or_else(|| BunnyChessApiError::UnauthorizedError("User Must Login".to_string()))
  }

  fn get_user_claims(&self) -> Result<UserClaims, BunnyChessApiError> {
    self
      .extensions()
      .get::<UserClaims>()
      .cloned()
      .ok_or_else(|| BunnyChessApiError::UnauthorizedError("User Must Login".to_string()))
  }
}
