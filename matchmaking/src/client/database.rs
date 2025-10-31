use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;

use crate::config::DatabaseConfig;

#[async_trait]
pub trait Database {
    type DB: sqlx::Database;

    async fn begin_tx(&self) -> Result<sqlx::Transaction<Self::DB>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PostgresDB {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDB {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPoolOptions::new()
                .max_connections(config.max_connections)
                .connect(config.db_url.as_str())
                .await?,
        })
    }

    pub async fn migrate(&self) {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .expect("Could not run migrations");
    }
}

#[async_trait]
impl Database for PostgresDB {
    type DB = sqlx::Postgres;

    async fn begin_tx(&self) -> Result<sqlx::Transaction<Self::DB>, sqlx::Error> {
        self.pool.begin().await
    }
}
