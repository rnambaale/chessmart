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
    // #[error("DB Error {0}")]
    // Db(#[from] sqlx::Error),

    #[error("Invalid amount: {0}")]
    InvalidAmount(String),

    #[error("Invalid quote {0}")]
    InvalidQuote(String),

    #[error("{0}")]
    SwapAmountMismatch(String),

    #[error("duplicate promises.")]
    SwapHasDuplicatePromises,

    #[error("Invalid quote uuid {0}")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Not Enough tokens. Required amount {0}")]
    NotEnoughTokens(u64),

    #[error("Proof already used {0}")]
    ProofAlreadyUsed(String),

    #[error("PrivateKey in keyset not found")]
    PrivateKeyNotFound,
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
