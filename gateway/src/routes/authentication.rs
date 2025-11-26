use axum::{
    extract::State, Json
};
use prost_types::Timestamp;
use shared::primitives::TimestampExt;
use tracing::{info, instrument};

use crate::{dtos::{request::{LoginRequestDto, RefreshTokenRequestDto, RegisterRequestDto}, response::{LoginResponseDto, MessageResponseDto, RegisterResponseDto}}, error::{AppResponseError, GatewayServiceError}, server::state::AppState, utils::claim::UserClaims};

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequestDto,
    responses(
        (status = 200, description = "post register", body = [RegisterResponseDto]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
)]
#[instrument(name = "post_register", skip(state), err)]
pub async fn post_register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, GatewayServiceError> {
    let RegisterRequestDto {
        email,
        username,
        password,
    } = request;

    let shared::Account {
        created_at,
        email,
        id,
        ..
    } = state
        .account_client.clone()
        .register(
            shared::RegisterRequest { email, username, password, is_admin: false }
        ).await?
        .into_inner();

    info!("Successfully register user: {id}");

    let response = RegisterResponseDto {
        id,
        email,
        created_at: created_at.unwrap().to_string()
    };

    Ok(Json(response))
}

#[utoipa::path(
    post,
    request_body = LoginRequestDto,
    path = "/auth/login",
    responses(
        (status = 200, description = "Success login user", body = [LoginResponseDto]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    )
)]
pub async fn login(
  State(state): State<AppState>,
  Json(request): Json<LoginRequestDto>,
) -> Result<Json<LoginResponseDto>, GatewayServiceError> {
    info!("Login user with request: {request:?}.");

    let LoginRequestDto { email, password} = request;

    let shared::LoginResponse {
        jwt,
        jwt_expires,
        jwt_refresh,
        jwt_refresh_expires,
    } = state
        .account_client.clone()
        .login(
            shared::LoginRequest { email, password }
        ).await?
        .into_inner();

    let jwt_expires = match jwt_expires {
        Some(jwt_expires) => Some(Timestamp::to_chrono(&jwt_expires)),
        None => None,
    };

    let jwt_refresh_expires = match jwt_refresh_expires {
        Some(jwt_refresh_expires) => Some(Timestamp::to_chrono(&jwt_refresh_expires)),
        None => None,
    };

    let repsonse = LoginResponseDto {
        jwt,
        jwt_expires,
        jwt_refresh,
        jwt_refresh_expires,
    };

    Ok(Json(repsonse))
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = 200, description = "Success get new access token and refresh token", body = [LoginResponseDto]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
)]
pub async fn refresh(
  State(state): State<AppState>,
  Json(request): Json<RefreshTokenRequestDto>,
) -> Result<Json<LoginResponseDto>, GatewayServiceError> {
    info!("Refresh token with request: {request:?}.");

    let RefreshTokenRequestDto { token } = request;

    let shared::LoginResponse {
        jwt,
        jwt_expires,
        jwt_refresh,
        jwt_refresh_expires,
    } = state
        .account_client.clone()
        .refresh(
            shared::RefreshRequest { jwt_refresh: token }
        ).await?
        .into_inner();

    let jwt_expires = match jwt_expires {
        Some(jwt_expires) => Some(Timestamp::to_chrono(&jwt_expires)),
        None => None,
    };

    let jwt_refresh_expires = match jwt_refresh_expires {
        Some(jwt_refresh_expires) => Some(Timestamp::to_chrono(&jwt_refresh_expires)),
        None => None,
    };

    let repsonse = LoginResponseDto {
        jwt,
        jwt_expires,
        jwt_refresh,
        jwt_refresh_expires,
    };

    info!("Success refresh token user response: {repsonse:?}.");

    Ok(Json(repsonse))
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = 200, description = "Success logout user", body = [MessageResponseDto]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn logout(
  State(_state): State<AppState>,
  user: UserClaims,
) -> Result<Json<MessageResponseDto>, GatewayServiceError> {
  info!("Logout user_id: {}", user.uid);
  todo!()
//   match services::authentication::logout(&state, user.uid).await {
//     Ok(_) => {
//       info!("Success logout user user_id: {}", user.uid);
//       Ok(Json(MessageResponseDto::new(
//         "This user has successfully logged out.",
//       )))
//     }
//     Err(e) => {
//       warn!("unsuccessfully logout user: {e:?}");
//       Err(e)
//     }
//   }
}
