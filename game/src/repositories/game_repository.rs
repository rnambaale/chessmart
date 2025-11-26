use redis::AsyncCommands;
use shared::error::BunnyChessApiError;

use crate::{primitives::ChessGame, state::state::AppState};

const UPDATE_GAME_SCRIPT: &str = include_str!("lua-scripts/update-game.lua");

pub async fn store_game(
    state: &AppState,
    chess_game: &ChessGame
) -> Result<(), BunnyChessApiError> {
    let game_key = get_game_key(chess_game.id.as_str());
    let mut connection = state.redis.get_multiplexed_async_connection().await?;

    let _: () = redis::pipe()
        .atomic()
        .hset(&game_key, "gameRepr", chess_game.to_string())
        .hset(&game_key, "seq", chess_game.seq())
        .expire(&game_key, 86400)
        .query_async(&mut connection)
        .await?;

    Ok(())
}

fn get_game_key(game_id: &str) -> String {
    format!("game:chess-game:{}:status", game_id)
}

pub async fn find_game(
    state: &AppState,
    game_id: &str
) -> Result<Option<ChessGame>, BunnyChessApiError> {
    let game_key = get_game_key(game_id);

    let mut connection = state.redis.get_multiplexed_async_connection().await?;

    let game_repr: Option<String> = connection.hget(&game_key, "gameRepr").await?;

    match game_repr {
        Some(repr) => {
            // let game = ChessGame::from_string(&repr)
            //     .map_err(|e| BunnyChessApiError::ParseJsonError(e.to_string()))?;
            let game = ChessGame::from_string(&repr)?;

            Ok(Some(game))
        }
        None => Ok(None)
    }
}

pub async fn delete_game(
    state: &AppState,
    game_id: &str
) -> Result<(), BunnyChessApiError> {
    let game_key = get_game_key(game_id);

    let mut connection = state.redis.get_multiplexed_async_connection().await?;

    let result: () = connection.del(&game_key).await?;

    Ok(result)
}

pub async fn update_game(
    state: &AppState,
    chess_game: &ChessGame
) -> Result<(), BunnyChessApiError> {
    let game_key = get_game_key(chess_game.id.as_str());

    let mut connection = state.redis.get_multiplexed_async_connection().await?;

    let game_state = chess_game.to_string();

    let script = redis::Script::new(UPDATE_GAME_SCRIPT);
    let result: i32 = script
        .key(&game_key)
        .arg(&game_state)
        .arg(chess_game.seq())
        .invoke_async(&mut connection)
        .await
        .map_err(|e| BunnyChessApiError::RedisError(e))?;

    match result {
        1 => Ok(()),
        0 => Err(BunnyChessApiError::ConcurrentMoveError(
            format!("Trying to update game {} with same seq number", chess_game.id)
        )),
        _ => Err(BunnyChessApiError::UnexpectedError(
            "Unexpected return value from Lua script".to_string()
        )),
    }
}
