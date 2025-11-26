use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameServiceError {
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error("Invalid move: {0}")]
    InvalidMove(String),

    #[error("Game not found: id={0}")]
    GameNotFoundError(String),

    #[error("{0}")]
    TurnError(String),

    #[error("Game is already over")]
    GameOverError,

    #[error("{0}")]
    UnexpectedError(String),

    #[error("{0}")]
    ConcurrentMoveError(String),

    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),

    #[error("{0}")]
    UnknownGameTypeError(String),
}

impl From<GameServiceError> for tonic::Status {
    fn from(error: GameServiceError) -> Self {
        tonic::Status::invalid_argument(error.to_string())
    }
}
