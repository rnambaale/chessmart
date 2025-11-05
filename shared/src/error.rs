use thiserror::Error;

#[derive(Error, Debug)]
pub enum BunnyChessApiError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("{0}")]
    UnknownGameTypeError(String),

    // #[error("Session not found error {0}")]
    // SessionNotFoundError(String),

    #[error("Invalid quote uuid {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("{0}")]
    InvalidInputError(String),

    #[error("{0}")]
    HashError(String),

    #[error("Task execution error: {0}")]
    SpawnTaskError(#[from] tokio::task::JoinError),

    // Auth errors
    #[error("{0}")]
    EmailNotFoundError(String),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),

    #[error("Session not found error {0}")]
    SessionNotFoundError(String),

    #[error("{0}")]
    InvalidSessionError(String),

    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    GrpcError(String),
}

impl From<BunnyChessApiError> for tonic::Status {
    fn from(error: BunnyChessApiError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}

impl From<tonic::transport::Error> for BunnyChessApiError {
    fn from(error: tonic::transport::Error) -> Self {
        eprintln!("gRPC error: {}", error);
        BunnyChessApiError::GrpcError(error.to_string())
    }
}

impl From<argon2::password_hash::Error> for BunnyChessApiError {
  fn from(value: argon2::password_hash::Error) -> Self {
    BunnyChessApiError::HashError(value.to_string())
  }
}
