use thiserror::Error;

#[derive(Error, Debug)]
pub enum BunnyChessApiError {
    #[error("DB Error {0}")]
    Db(#[from] sqlx::Error),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    NatsError(#[from] async_nats::Error),

    #[error("{0}")]
    UnknownGameTypeError(String),

    #[error("{0}")]
    GameNotFoundError(String),

    #[error("{0}")]
    InvalidInputError(String),

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

