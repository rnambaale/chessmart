use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;

use crate::{database::{self, postgres::PostgresDB}, error::BunnyChessApiError};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,

    // #[serde(skip_serializing)]
    #[serde(skip)]
    pub password: String,

    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}


#[instrument(level = "debug", err)]
pub async fn get_by_email(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    email: &str
) -> Result<User, BunnyChessApiError> {
    match sqlx::query!(
        "SELECT id, username, email, password, last_login_at, created_at, updated_at FROM users WHERE email = $1",
        email
    )
    .map(|row| User {
        id: row.id,
        username: row.username,
        email: row.email,
        password: row.password,
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

#[instrument(level = "debug", err)]
pub async fn get_by_id(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    id: &Uuid
) -> Result<User, BunnyChessApiError> {
    match sqlx::query!(
        "SELECT id, username, email, password, last_login_at, created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .map(|row| User {
        id: row.id,
        username: row.username,
        email: row.email,
        password: row.password,
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

#[instrument(level = "debug", err)]
pub async fn update_last_login(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    user: &User,
) -> Result<(), BunnyChessApiError> {
    sqlx::query!(
        "UPDATE users SET last_login_at = $1 WHERE id = $2",
        Utc::now(),
        user.id
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}
