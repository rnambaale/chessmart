use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChessmartApiError {
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

impl From<ChessmartApiError> for tonic::Status {
    fn from(error: ChessmartApiError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}

impl From<tonic::transport::Error> for ChessmartApiError {
    fn from(error: tonic::transport::Error) -> Self {
        eprintln!("gRPC error: {}", error);
        ChessmartApiError::GrpcError(error.to_string())
    }
}

