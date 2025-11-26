use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthServiceError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("{0}")]
    InvalidInputError(String),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("{0}")]
    EmailNotFoundError(String),

    #[error("Invalid quote uuid {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("{0}")]
    HashError(String),

    #[error("Task execution error: {0}")]
    SpawnTaskError(#[from] tokio::task::JoinError),

    #[error("Session not found error {0}")]
    SessionNotFoundError(String),

    #[error("{0}")]
    InvalidSessionError(String),

    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

impl From<AuthServiceError> for tonic::Status {
    fn from(error: AuthServiceError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for AuthServiceError {
  fn from(value: argon2::password_hash::Error) -> Self {
    AuthServiceError::HashError(value.to_string())
  }
}
