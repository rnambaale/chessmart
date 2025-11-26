use chrono::{Duration, Utc};
use tracing::info;
use uuid::Uuid;

use crate::{client::database::Database, constants::{ACCESS_TOKEN_ENCODE_KEY, EXPIRE_BEARER_TOKEN_SECS, EXPIRE_REFRESH_TOKEN_SECS, REFRESH_TOKEN_DECODE_KEY, REFRESH_TOKEN_ENCODE_KEY}, dtos::{request::RefreshTokenRequestDto, response::LoginResponseDto}, error::AuthServiceError, state::state::AppState, utils::claim::UserClaims};

pub fn generate_tokens(
  user_id: Uuid,
  session_id: Uuid,
) -> Result<LoginResponseDto, AuthServiceError> {
    let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id)
        .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
    let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id)
        .encode(&REFRESH_TOKEN_ENCODE_KEY)?;

    let jwt_expires = Utc::now() + Duration::try_seconds(EXPIRE_BEARER_TOKEN_SECS.as_secs() as i64).expect("invalid duration");
    let jwt_fresh_expires = Utc::now() + Duration::try_seconds(EXPIRE_REFRESH_TOKEN_SECS.as_secs() as i64).expect("invalid duration");

    Ok(LoginResponseDto::new(
        access_token,
        refresh_token,
        jwt_expires,
        jwt_fresh_expires,
    ))
}

pub async fn refresh(state: &AppState, req: &RefreshTokenRequestDto) -> Result<LoginResponseDto, AuthServiceError> {
    let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;

    info!("Refresh token: {user_claims:?}");
    let user_id = crate::services::session::check(&state.redis, &user_claims).await?;

    let mut tx = state.db.begin_tx().await?;
    let user = crate::repositories::user::get_by_id(&mut tx, &user_id).await?;
    tx.commit().await?;

    let session_id = crate::services::session::set(&state.redis, user.id).await?;
    info!("Set new session for user: {}", user.id);
    let resp = generate_tokens(user.id, session_id)?;
    info!("Refresh token success: {user_claims:?}");
    Ok(resp)
}
