use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde_json::json;
use thiserror::Error;
use tracing::{event, Level};

#[derive(Error, Debug)]
pub enum BunnyChessApiError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error("{0}")]
    HashError(String),

    #[error("Invalid quote uuid {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Task execution error: {0}")]
    SpawnTaskError(#[from] tokio::task::JoinError),

    #[error("{0}")]
    InvalidInputError(String),
}

impl IntoResponse for BunnyChessApiError {
    fn into_response(self) -> Response {
        event!(Level::ERROR, "error in API server: {:?}", self);

        let body = Json(json!({
            "code": 0,
            "detail": self.to_string(),
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl From<argon2::password_hash::Error> for BunnyChessApiError {
  fn from(value: argon2::password_hash::Error) -> Self {
    BunnyChessApiError::HashError(value.to_string())
  }
}
