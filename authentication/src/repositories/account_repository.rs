use chrono::{DateTime, Utc};
use shared::{error::BunnyChessApiError};
use uuid::Uuid;

use crate::database::{Database, postgres::PostgresDB};

pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub username: String,

    // #[serde(skip_serializing)]
    // #[serde(skip)]
    pub password: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

pub struct AccountRepository {
    database: PostgresDB,
}

impl AccountRepository {
    pub fn new(database: PostgresDB) -> Self {
        Self { database }
    }

    pub async fn insert_account(
        &self,
        account: &Account,
    ) -> Result<Account, BunnyChessApiError> {
        let mut tx = self.database.begin_tx().await?;

        sqlx::query!(
            "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
            account.id,
            account.username,
            account.email,
            account.password
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        todo!()
    }
}


/*
#[instrument(level = "debug", err)]
pub async fn save(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    user: &User,
) -> Result<(), BunnyChessApiError> {
    sqlx::query!(
        "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
        user.id,
        user.username,
        user.email,
        user.password
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

*/
