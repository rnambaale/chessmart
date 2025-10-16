use tracing::info;
use uuid::Uuid;

use crate::{database::{self, postgres::PostgresDB, user::User, Database}, error::BunnyChessApiError, routes::authentication::RegisterRequestDto, server::state::AppState};

pub async fn register(state: AppState, req: &RegisterRequestDto) -> Result<Uuid, BunnyChessApiError> {
    info!("Register a new user request: {req:?}.");
    let mut tx = state.db.begin_tx().await?;
    check_unique_username(&mut tx, &req.username).await?;
    check_unique_email(&mut tx, &req.email).await?;

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        email: req.email.to_string(),
        username: req.username.to_string(),
        password: req.password.to_string()
    };

    database::user::save(&mut tx, &user).await?;
    tx.commit().await?;
    Ok(user_id)
}

async fn check_unique_username(
  tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
  username: &str,
) -> Result<bool, BunnyChessApiError> {
    let username_result = database::user::get_by_username(tx, username).await;

    let username_exists = match username_result {
        Ok(_) => true,
        Err(BunnyChessApiError::Db(sqlx::Error::RowNotFound)) => false,
        Err(e) => return Err(e),
    };

    if username_exists {
        return Err(BunnyChessApiError::UserAlreadyExists);
    }

    Ok(true)
}

pub async fn check_unique_email(
  tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
  email: &str,
) -> Result<bool, BunnyChessApiError> {
    let email_result = database::user::get_by_email(tx, email).await;

    let email_exists = match email_result {
        Ok(_) => true,
        Err(BunnyChessApiError::Db(sqlx::Error::RowNotFound)) => false,
        Err(e) => return Err(e),
    };

    if email_exists {
        return Err(BunnyChessApiError::UserAlreadyExists);
    }

    Ok(true)
}

