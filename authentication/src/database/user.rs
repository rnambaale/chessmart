use chrono::{DateTime, Utc};
use shared::error::BunnyChessApiError;
use tracing::instrument;
use uuid::Uuid;

use crate::database::{self, postgres::PostgresDB};

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

#[instrument(level = "debug", err)]
pub async fn get_by_id(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    id: &Uuid
) -> Result<Account, BunnyChessApiError> {
    match sqlx::query!(
        "SELECT id, username, email, password, is_admin, last_login_at, created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .map(|row| Account {
        id: row.id,
        username: row.username,
        email: row.email,
        password: row.password,
        is_admin: row.is_admin,
        last_login_at: row.last_login_at,
        created_at: row.created_at.unwrap(),
        updated_at: row.updated_at.unwrap(),
    })
    .fetch_one(&mut **tx)
    .await {
        Ok(user) => Ok(user),
        Err(e) => Err(e.into())
    }
}

pub async fn insert_account(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as crate::database::Database>::DB>,
    account: &Account,
) -> Result<(), BunnyChessApiError> {
    sqlx::query!(
        "INSERT INTO users (id, username, email, password) VALUES ($1, $2, $3, $4)",
        account.id,
        account.username,
        account.email,
        account.password
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn find_account_by_email(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as crate::database::Database>::DB>,
    email: &str,
) -> Result<Option<Account>, BunnyChessApiError> {
    match sqlx::query!(
        "SELECT id, username, email, password, is_admin, last_login_at, created_at, updated_at FROM users WHERE email = $1",
        email
    )
    .map(|row| Account {
        id: row.id,
        username: row.username,
        email: row.email,
        is_admin: row.is_admin,
        password: row.password,
        last_login_at: row.last_login_at,
        created_at: row.created_at.unwrap(),
        updated_at: row.updated_at.unwrap(),
    })
    .fetch_optional(&mut **tx)
    .await {
        Ok(user) => Ok(user),
        Err(e) => Err(e.into())
    }
}

pub async fn update_last_login(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as crate::database::Database>::DB>,
    account: &Account,
) -> Result<(), BunnyChessApiError> {
    sqlx::query!(
        "UPDATE users SET last_login_at = $1 WHERE id = $2",
        Utc::now(),
        account.id
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
