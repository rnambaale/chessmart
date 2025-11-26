use thiserror::Error;

#[derive(Error, Debug)]
pub enum MatchmakingServiceError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("{0}")]
    UnknownGameTypeError(String),

    #[error("Invalid quote uuid {0}")]
    InvalidUuid(#[from] uuid::Error),
}

impl From<MatchmakingServiceError> for tonic::Status {
    fn from(error: MatchmakingServiceError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}
