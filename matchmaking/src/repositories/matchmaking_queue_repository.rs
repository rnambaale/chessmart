use redis::{RedisResult, Script, ToRedisArgs};
use shared::error::BunnyChessApiError;
use shared::primitives::GameType;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::repositories::player_status_repository::PlayerStatusRepositoryService;

pub enum PlayerStatus {
    Undefined,
    Searching,
    Pending,
    Playing
}

impl PlayerStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            PlayerStatus::Undefined => "undefined",
            PlayerStatus::Searching => "searching",
            PlayerStatus::Pending => "pending",
            PlayerStatus::Playing => "playing",
        }
    }
}

impl ToRedisArgs for PlayerStatus {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        // Use string representation
        out.write_arg(self.as_str().as_bytes());

        // Or use numeric representation:
        // out.write_arg_fmt(self.as_i32());
    }
}

impl FromStr for PlayerStatus {
    type Err = BunnyChessApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undefined" => Ok(PlayerStatus::Undefined),
            "searching" => Ok(PlayerStatus::Searching),
            "pending" => Ok(PlayerStatus::Pending),
            "playing" => Ok(PlayerStatus::Playing),
            _ => Err(BunnyChessApiError::UnknownGameTypeError(s.into())),
        }
    }
}

pub struct QueueKeys {
   pub queue_key: String,
   pub times_key: String,
}


// Trait defining matchmaking operations
#[async_trait::async_trait]
pub trait MatchmakingQueue: Send + Sync {
    // async fn match_players(
    //     &self,
    //     queue_key: &str,
    //     times_key: &str,
    //     base_mmr_range: i32,
    //     mmr_increase_per_second: f64,
    //     max_mmr_delta: i32,
    // ) -> RedisResult<Vec<String>>;

    async fn add_player_to_queue(
        &self,
        account_id: &str,
        mmr: u16,
        game_type: &GameType,
        ranked: bool
    ) -> RedisResult<()>;

    async fn remove_player_from_queue(
        &self,
        account_id: &str,
        game_type: GameType,
        ranked: bool,
    ) -> RedisResult<i32>;
}

// pub const MATCH_PLAYERS_SCRIPT: &str = include_str!("lua-scripts/match-players.lua");
pub const ADD_PLAYER_TO_QUEUE_SCRIPT: &str = include_str!("lua-scripts/add-player-to-queue.lua");
pub const REMOVE_PLAYER_FROM_QUEUE_SCRIPT: &str = include_str!("lua-scripts/remove-player-from-queue.lua");

// Redis implementation
pub struct RedisMatchmakingQueue {
    client: redis::Client,
    scripts: Arc<Mutex<HashMap<String, Script>>>,
}

impl RedisMatchmakingQueue {
    pub fn new(client: redis::Client) -> Self {
        let mut scripts = HashMap::new();

        // Pre-define scripts (similar to NestJS constructor)
        // scripts.insert(
        //     "matchPlayers".to_string(),
        //     Script::new(MATCH_PLAYERS_SCRIPT),
        // );
        scripts.insert(
            "addPlayerToQueue".to_string(),
            Script::new(ADD_PLAYER_TO_QUEUE_SCRIPT),
        );
        scripts.insert(
            "removePlayerFromQueue".to_string(),
            Script::new(REMOVE_PLAYER_FROM_QUEUE_SCRIPT),
        );

        Self {
            client,
            scripts: Arc::new(Mutex::new(scripts)),
        }
    }

    fn get_queue_keys(game_type: &GameType, ranked: bool) -> QueueKeys {
        let ranked_value = match ranked {
            true => "ranked",
            false => "normal"
        };

        QueueKeys {
            queue_key: format!("matchmaking:queue:{}:{}", game_type.to_str(), ranked_value),
            times_key: format!("matchmaking:queue:{}:{}:times", game_type.to_str(), ranked_value)
        }
    }
}

#[async_trait::async_trait]
impl MatchmakingQueue for RedisMatchmakingQueue {
    // async fn match_players(
    //     &self,
    //     queue_key: &str,
    //     times_key: &str,
    //     base_mmr_range: i32,
    //     mmr_increase_per_second: f64,
    //     max_mmr_delta: i32,
    // ) -> RedisResult<Vec<String>> {
    //     let scripts = self.scripts.lock().await;
    //     let script = scripts.get("matchPlayers")
    //         .ok_or_else(|| redis::RedisError::from((
    //             redis::ErrorKind::TypeError,
    //             "matchPlayers script not found",
    //         )))?;

    //     let result: Vec<String> = script
    //         .key(queue_key)
    //         .key(times_key)
    //         .arg(base_mmr_range)
    //         .arg(mmr_increase_per_second)
    //         .arg(max_mmr_delta)
    //         .invoke_async(&mut self.client.get_async_connection().await?)
    //         .await?;

    //     Ok(result)
    // }

    async fn add_player_to_queue(
        &self,
        account_id: &str,
        mmr: u16,
        game_type: &GameType,
        ranked: bool
    ) -> RedisResult<()> {
        let scripts = self.scripts.lock().await;
        let script = scripts.get("addPlayerToQueue")
            .ok_or_else(|| redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "addPlayerToQueue script not found",
            )))?;

        let queue_keys = Self::get_queue_keys(&game_type, ranked);
        let account_status_key = PlayerStatusRepositoryService::get_account_status_key(account_id);
        let player_status = PlayerStatus::Searching;

        script
            .key(&queue_keys.queue_key)
            .key(&queue_keys.times_key)
            .key(&account_status_key)
            .arg(player_status.as_str())
            .arg(account_id)
            .arg(mmr)
            .arg(ranked)
            .invoke_async(&mut self.client.get_multiplexed_async_connection().await?)
            .await?;

        Ok(())
    }

    async fn remove_player_from_queue(
        &self,
        account_id: &str,
        game_type: GameType,
        ranked: bool,
    ) -> RedisResult<i32> {

        let account_status_key = PlayerStatusRepositoryService::get_account_status_key(account_id);
        let queue_keys = Self::get_queue_keys(&game_type, ranked);

        let scripts = self.scripts.lock().await;
        let script = scripts.get("removePlayerFromQueue")
            .ok_or_else(|| redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "removePlayerFromQueue script not found",
            )))?;

        let result: i32 = script
            .key(&queue_keys.queue_key)
            .key(&queue_keys.times_key)
            .key(&account_status_key)
            .arg(account_id)
            .arg(game_type)
            .arg(ranked)
            // .invoke_async(&mut self.client.get_async_connection().await?)
            .invoke_async(&mut self.client.get_multiplexed_async_connection().await?)
            .await?;

        Ok(result)
    }
}
