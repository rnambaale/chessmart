use rand::{seq::SliceRandom, thread_rng};
use shared::{events::{GameOverEvent, GameStartEvent, GameStateUpdateEvent}};
use tracing::debug;

use crate::{error::GameServiceError, jobs::{TaskMessage, TaskType, check_game_job::CheckGamePayload}, primitives::{AccountIds, ChessGame, CreateGameDto}, state::state::AppState};

pub async fn create_game(
    state: &AppState,
    payload: CreateGameDto,
) -> Result<ChessGame, GameServiceError> {
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

async fn add_game_to_check_queue(state: &AppState, chess_game: &ChessGame) -> Result<(), GameServiceError> {
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
) -> Result<ChessGame, GameServiceError> {
    let game_option = crate::repositories::game_repository::find_game(
        state,
        game_id
    ).await?;

    match game_option {
        Some(game) => Ok(game),
        None => Err(GameServiceError::GameNotFoundError(format!("Couldn't find game {}", game_id)))
    }
}

pub async fn check_game_result(
    state: &AppState,
    chess_game: &ChessGame,
) -> Result<(), GameServiceError> {
    let game_result = chess_game.check_game_result()?;
    if game_result.is_none() {
        return Ok(());
    }

    let result = game_result.unwrap();

    let ChessGame {
        account_ids,
        id,
        game_type,
        metadata,
        ..
    } = chess_game;

    crate::services::streaming_service::emit_game_over_event(
        state,
        GameOverEvent {
            account_id_0: account_ids.w.to_owned(),
            account_id_1: account_ids.b.to_owned(),
            outcome: result.outcome.to_str().to_owned(),
            game_over_reason: result.reason.to_str().to_owned(),
            winner_account_id: result.winner_account_id,
            game_id: id.to_string(),
            game_type: game_type.to_str().to_owned(),
            metadata: metadata.to_string()
        }
    ).await?;

    debug!("Game {}: emitted game over event", chess_game.id);

    remove_game_from_check_queue(state, chess_game).await?;

    crate::repositories::game_repository::delete_game(
        state,
        &chess_game.id
    ).await?;

    Ok(())
}

async fn remove_game_from_check_queue(state: &AppState, chess_game: &ChessGame) -> Result<(), GameServiceError> {
  let ChessGame { id, ..} = chess_game;
  let body = CheckGamePayload { game_id: id.to_string() };
  let message_type = TaskType::CheckGameJob(body);
  let message = TaskMessage { task: message_type };
  let serialized_message = bincode::serialize(&message).unwrap();

    let mut conn = state.redis.get_connection()?;
    let _: () = redis::cmd("LPOP")
        .arg(crate::jobs::check_game_job::CHECK_GAME_QUEUE_NAME)
        .arg(serialized_message.clone())
        .query(&mut conn)?;
    Ok(())
}

pub async fn make_move(
    state: &AppState,
    game_id: &str,
    account_id: &str,
    game_move: &str,
) -> Result<ChessGame, GameServiceError> {
    let chess_game = get_game(state, game_id).await?;
    let _ = chess_game.make_move(account_id, game_move)?;

    crate::repositories::game_repository::update_game(
        state,
        &chess_game
    ).await?;

    crate::services::streaming_service::emit_game_state_update(
        state,
        GameStateUpdateEvent {
            account_id: account_id.to_string(),
            r#move: game_move.to_owned(),
            game_id: chess_game.id.to_owned(),
            // fen: chess_game.fen,
            seq: chess_game.seq(),
            clocks: shared::events::ClockState { w: chess_game.game_clocks.w as u32, b: chess_game.game_clocks.b as u32 }
        }
    ).await?;

    check_game_result(state, &chess_game).await?;

    debug!("Game {}: move '{}' by {}", game_id, game_move, account_id);

    Ok(chess_game)
}
