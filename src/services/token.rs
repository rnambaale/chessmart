use uuid::Uuid;

use crate::{constants::{ACCESS_TOKEN_ENCODE_KEY, EXPIRE_BEARER_TOKEN_SECS, EXPIRE_REFRESH_TOKEN_SECS, REFRESH_TOKEN_ENCODE_KEY}, dtos::response::LoginResponseDto, error::BunnyChessApiError, utils::claim::UserClaims};

pub fn generate_tokens(
  user_id: Uuid,
  session_id: Uuid,
) -> Result<LoginResponseDto, BunnyChessApiError> {
    let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id)
        .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
    let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id)
        .encode(&REFRESH_TOKEN_ENCODE_KEY)?;

    Ok(LoginResponseDto::new(
        access_token,
        refresh_token,
        EXPIRE_BEARER_TOKEN_SECS.as_secs(),
        EXPIRE_REFRESH_TOKEN_SECS.as_secs()
    ))
}
