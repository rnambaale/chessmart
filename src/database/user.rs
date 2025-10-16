use tracing::instrument;
use uuid::Uuid;

use crate::{database::{self, postgres::PostgresDB}, error::BunnyChessApiError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,

    // #[serde(skip_serializing)]
    // #[serde(skip)] // Skip both serialization and deserialization
    pub password: String,

    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

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

// #[tracing::instrument]
#[instrument(level = "debug", err)]
pub async fn get_by_username(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    username: &str
) -> Result<User, BunnyChessApiError> {
    let user: User = sqlx::query!(
        "SELECT id, username, email, password FROM users WHERE username = $1",
        username
    )
    .map(|row| User {
        id: row.id,
        username: row.username,
        email: row.email,
        password: row.password,
        // created_at: row.created_at,
        // updated_at: row.updated_at
    })
    // .fetch_one(tx)
    .fetch_one(&mut **tx)
    .await?;

    Ok(user)
}

// pub async fn get_by_username(
//     pool: &sqlx::Pool<sqlx::Postgres>,
//     username: &str
// ) -> Result<User, BunnyChessApiError> {
//     match sqlx::query!(
//         "SELECT id, username, email, password FROM users WHERE username = $1",
//         username
//     )
//     .map(|row| User {
//         id: row.id,
//         username: row.username,
//         email: row.email,
//         password: row.password,
//     })
//     .fetch_one(pool)
//     .await {
//         Ok(user) => Ok(user),
//         // Err(sqlx::Error::RowNotFound) => {
//         //     Err(BunnyChessApiError::UserNotFound)
//         // },
//         Err(e) => Err(e.into())
//     }
// }

#[instrument(level = "debug", err)]
pub async fn get_by_email(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as database::Database>::DB>,
    email: &str
) -> Result<User, BunnyChessApiError> {
    match sqlx::query!(
        "SELECT id, username, email, password FROM users WHERE email = $1",
        email
    )
    .map(|row| User {
        id: row.id,
        username: row.username,
        email: row.email,
        password: row.password,
    })
    .fetch_one(&mut **tx)
    .await {
        Ok(user) => Ok(user),
        Err(e) => Err(e.into())
    }
}
