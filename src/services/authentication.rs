use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::{database::{self, postgres::PostgresDB, user::User, Database}, dtos::{request::{LoginRequestDto, RegisterRequestDto}, response::LoginResponseDto}, error::BunnyChessApiError, server::state::AppState, services, utils};

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
        password: utils::password::hash(req.password.to_string()).await?,
        last_login_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
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

pub async fn login(state: &AppState, req: &LoginRequestDto) -> Result<LoginResponseDto, BunnyChessApiError> {
    info!("User login request :{req:?}.");

    let mut tx = state.db.begin_tx().await?;
    let user = database::user::get_by_email(&mut tx, &req.email).await?;

    utils::password::verify(req.password.clone(), user.password.clone()).await?;

    database::user::update_last_login(&mut tx, &user).await?;

    tx.commit().await?;

    let session_id = services::session::set(&state.redis, user.id).await?;
    let resp = services::token::generate_tokens(user.id, session_id)?;
    Ok(resp)
}
