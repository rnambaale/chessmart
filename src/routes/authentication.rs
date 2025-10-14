use axum::{
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use utoipa::ToSchema;

use crate::error::BunnyChessApiError;

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
        (status = 200, description = "post register", body = [RegisterResponseDto])
    ),
)]
#[instrument(name = "post_register", err)]
pub async fn post_register(
    Json(register_request): Json<RegisterRequestDto>,
) -> Result<Json<RegisterResponseDto>, BunnyChessApiError> {

    // TODO: Perform the actual registration

    let now = Utc::now().timestamp() as u64;

    Ok(Json(RegisterResponseDto {
        id: String::from("111"),
        email: register_request.email,
        created_at: now.to_string()
    }))
}
