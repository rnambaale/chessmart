use axum::{
    extract::State, Json
};
use chrono::Utc;
use tracing::{info, instrument, warn};

use crate::{dtos::{request::{LoginRequestDto, RegisterRequestDto}, response::{LoginResponseDto, RegisterResponseDto}}, error::{AppResponseError, BunnyChessApiError}, server::state::AppState, services};

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
    Json(req): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, BunnyChessApiError> {
    match services::authentication::register(state, &req).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let now = Utc::now().timestamp() as u64;
            let resp = RegisterResponseDto {
                id: user_id.to_string(),
                email: req.email,
                created_at: now.to_string()
            };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Error encountered while registering user: {e:?}");
            Err(e)
        }
    }
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
  Json(req): Json<LoginRequestDto>,
) -> Result<Json<LoginResponseDto>, BunnyChessApiError> {
    todo!();
    // info!("Login user with request: {req:?}.");
    // match service::authentication::login(&state, &req).await {
    //     Ok(resp) => {
    //         info!("Success login user_id: {resp:?}.");
    //         Ok(Json(resp))
    //     }
    //     Err(e) => {
    //         warn!("Unsuccessfully login user error: {e:?}.");
    //         Err(e)
    //     }
    // }
}
