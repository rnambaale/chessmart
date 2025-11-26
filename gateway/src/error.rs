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
pub enum GatewayServiceError {
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    NatsError(#[from] async_nats::Error),

    #[error("Task execution error: {0}")]
    SpawnTaskError(#[from] tokio::task::JoinError),

    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    InvalidSessionError(String),

    #[error("Session not found error {0}")]
    SessionNotFoundError(String),

    #[error("{0}")]
    UnauthorizedError(String),

    #[error("{0}")]
    GrpcError(String),
}

impl IntoResponse for GatewayServiceError {
    fn into_response(self) -> Response {
        event!(Level::ERROR, "error in API server: {:?}", self);

        let body = Json(json!({
            "code": 0,
            "detail": self.to_string(),
        }));

        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

impl GatewayServiceError {
  pub fn response(self) -> (StatusCode, AppResponseError) {
    use GatewayServiceError::*;
    let message = self.to_string();


    let (kind, code, details, status_code) = match self {
      RedisError(_err) => (
        "REDIS_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      NatsError(_err) => (
        "NATS_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SpawnTaskError(_err) => (
        "SPAWN_TASK_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ParseJsonError(_err) => (
        "PARSE_JSON_ERROR".to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      JwtError(_err) => (
        "UNAUTHORIZED_ERROR".to_string(),
        None,
        vec![],
        StatusCode::UNAUTHORIZED,
      ),
      InvalidSessionError(_err) => (
        "INVALID_SESSION_ERROR".to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      SessionNotFoundError(_err) => (
        "SESSION_NOT_FOUND_ERROR".to_string(),
        None,
        vec![],
        StatusCode::NOT_FOUND,
      ),
      UnauthorizedError(_err) => (
        "UNAUTHORIZED_ERROR".to_string(),
        None,
        vec![],
        StatusCode::UNAUTHORIZED,
      ),
      GrpcError(_err) => (
        "INTERNAL_SERVER_ERROR".to_string(),
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

impl From<tonic::Status> for GatewayServiceError {
    fn from(status: tonic::Status) -> Self {
        eprintln!("gRPC error: {}", status);
        GatewayServiceError::GrpcError(status.message().to_string())
    }
}

impl From<tonic::transport::Error> for GatewayServiceError {
    fn from(error: tonic::transport::Error) -> Self {
        eprintln!("gRPC error: {}", error);
        GatewayServiceError::GrpcError(error.to_string())
    }
}
