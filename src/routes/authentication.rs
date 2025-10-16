use axum::{
    extract::State, Json
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use utoipa::ToSchema;

use crate::{error::{AppResponseError, BunnyChessApiError}, server::state::AppState, services};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequestDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct RegisterResponseDto {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

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
