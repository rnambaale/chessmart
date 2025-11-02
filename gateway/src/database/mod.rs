use async_trait::async_trait;

pub mod postgres;

#[async_trait]
pub trait Database {
    type DB: sqlx::Database;

    async fn begin_tx(&self) -> Result<sqlx::Transaction<Self::DB>, sqlx::Error>;
}
