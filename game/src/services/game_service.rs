use rand::{seq::SliceRandom, thread_rng};
use shared::{error::BunnyChessApiError, events::GameStartEvent};
use tracing::debug;

use crate::{jobs::{TaskMessage, TaskType, check_game_job::CheckGamePayload}, primitives::{AccountIds, ChessGame, CreateGameDto}, state::state::AppState};

pub async fn create_game(
    state: &AppState,
    payload: CreateGameDto,
) -> Result<ChessGame, BunnyChessApiError> {
    let CreateGameDto {
        account_id0,
        account_id1,
        metadata,
        game_type
    } = payload;

    // Randomly assign colors
    let mut vec = vec![account_id0, account_id1];
    vec.shuffle(&mut thread_rng());

    let (white_account_id, black_account_id) = (vec[0].as_str(), vec[1].as_str());

    let chess_game = ChessGame::from_scratch(
        game_type,
        AccountIds {
            w: white_account_id.to_string(),
            b: black_account_id.to_string(),
        },
        metadata,
        crate::primitives::get_game_rules(game_type)
    );

    crate::repositories::game_repository::store_game(state, &chess_game).await?;

    crate::services::streaming_service::emit_game_start(
        state,
        GameStartEvent {
            account_id_0: black_account_id.to_owned(),
            account_id_1: black_account_id.to_string(),
            game_id: chess_game.id.to_owned()
        }
    ).await?;

    add_game_to_check_queue(state, &chess_game).await?;

    debug!(
      "Game {} ({}) created, w: {}, b: {}",
      chess_game.id,
      game_type.to_str(),
      white_account_id,
      black_account_id
    );

    Ok(chess_game)
}

async fn add_game_to_check_queue(state: &AppState, chess_game: &ChessGame) -> Result<(), BunnyChessApiError> {
  let ChessGame { id, ..} = chess_game;
  let body = CheckGamePayload { game_id: id.to_string() };
  let message_type = TaskType::CheckGameJob(body);
  let message = TaskMessage { task: message_type };
  let serialized_message = bincode::serialize(&message).unwrap();

    let mut conn = state.redis.get_connection()?;
    let _: () = redis::cmd("LPUSH")
        .arg(crate::jobs::check_game_job::CHECK_GAME_QUEUE_NAME)
        .arg(serialized_message.clone())
        .query(&mut conn)?;
    Ok(())
}

pub async fn get_game(
    state: &AppState,
    game_id: &str,
) -> Result<ChessGame, BunnyChessApiError> {
    let game_option = crate::repositories::game_repository::find_game(
        state,
        game_id
    ).await?;

    match game_option {
        Some(game) => Ok(game),
        None => Err(BunnyChessApiError::GameNotFoundError(format!("Couldn't find game {}", game_id)))
    }
}
