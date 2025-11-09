use redis::AsyncCommands;
use shared::error::BunnyChessApiError;

use crate::{primitives::ChessGame, state::state::AppState};

// const UPDATE_GAME_SCRIPT: &str = include_str!("lua-scripts/update-game.lua");

pub async fn store_game(
    state: &AppState,
    chess_game: &ChessGame
) -> Result<(), BunnyChessApiError> {
    let game_key = get_game_key(chess_game.id.as_str());
    // let mut connection = state.redis.connection.clone();
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
