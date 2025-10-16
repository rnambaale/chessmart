use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use tracing::{event, Level};

#[derive(Error, Debug)]
pub enum BunnyChessApiError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

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

impl BunnyChessApiError {
  pub fn response(self) -> (StatusCode, AppResponseError) {
    use BunnyChessApiError::*;
    let message = self.to_string();


    let (kind, code, details, status_code) = match self {
      Db(_err) => (
        "DB_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      RedisError(_err) => (
        "REDIS_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      UserAlreadyExists => (
        format!("USER_ALREADY_EXISTS_ERROR"),
        None,
        vec![],
        StatusCode::CONFLICT,
      ),
      InvalidInputError(_err) => (
        "INVALID_INPUT_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      SpawnTaskError(_err) => (
        "SPAWN_TASK_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      InvalidUuid(_err) => (
        "UUID_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      HashError(_err) => (
        "HASH_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
    };

    (
      status_code,
      AppResponseError::new(kind, message, code, details),
    )
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
  pub kind: String,
  pub error_message: String,
  pub code: Option<i32>,
  pub details: Vec<(String, String)>,
}

impl AppResponseError {
  pub fn new(
    kind: impl Into<String>,
    message: impl Into<String>,
    code: Option<i32>,
    details: Vec<(String, String)>,
  ) -> Self {
    Self {
      kind: kind.into(),
      error_message: message.into(),
      code,
      details,
    }
  }
}
