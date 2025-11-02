// pub async fn logout(state: &AppState, user_id: Uuid) -> Result<(), BunnyChessApiError> {
//     info!("Logout user id: {user_id}");
//     let key = SessionKey { user_id };
//     services::redis::del(&state.redis, &key).await?;
//     Ok(())
// }
