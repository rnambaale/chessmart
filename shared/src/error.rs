use thiserror::Error;

#[derive(Error, Debug)]
pub enum BunnyChessApiError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("{0}")]
    UnknownGameTypeError(String),
}

impl From<BunnyChessApiError> for tonic::Status {
    fn from(error: BunnyChessApiError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}
