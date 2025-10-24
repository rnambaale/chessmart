use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{database::{self, user::User, Database}, error::BunnyChessApiError, server::state::AppState};

pub async fn get_account_status(state: AppState, user_id: &Uuid) -> Result<User, BunnyChessApiError> {
    info!("Get account status with id: {user_id}");
    let mut tx = state.db.begin_tx().await?;
    let user = database::user::get_by_id(&mut tx, user_id).await?;
    tx.commit().await?;
    Ok(user)
}

#[derive(Serialize, Deserialize)]
pub enum PlayerStatus {
    Undefined,
    Searching,
    Pending,
    Playing
}

impl PlayerStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            PlayerStatus::Undefined => "undefined",
            PlayerStatus::Searching => "searching",
            PlayerStatus::Pending => "pending",
            PlayerStatus::Playing => "playing",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum GameType {
    Rapid10_0,
    Blitz5_3,
    Blitz5_0,
    Blitz3_2,
    Blitz3_0,
    Bullet1_0,
}

impl GameType {
    pub fn to_str(&self) -> &'static str {
        match self {
            GameType::Rapid10_0 => "Rapid10_0",
            GameType::Blitz5_3 => "Blitz5_3",
            GameType::Blitz5_0 => "Blitz5_0",
            GameType::Blitz3_2 => "Blitz3_2",
            GameType::Blitz3_0 => "Blitz3_0",
            GameType::Bullet1_0 => "Bullet1_0",
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MatchMakingStatus {
    pub status: PlayerStatus,
    pub game_type: Option<GameType>,
    pub ranked: Option<bool>,
    pub game_id: Option<String>
}

// pub async fn get_player_status(state: AppState, user_id: &Uuid) -> Result<MatchMakingStatus, BunnyChessApiError> {
//     info!("Get account status with id: {user_id}");
//     let account_key = get_account_status_key(&user_id);
//     match services::redis::hgetall(&state.redis, &account_key).await? {
//         Some(result) => todo!(),
//         None => Ok(
//             MatchMakingStatus {
//                 status: PlayerStatus::Undefined,
//                 game_type: None,
//                 ranked: None,
//                 game_id: None
//             }
//         )
//     }
//     // todo!()
// }

#[derive(Serialize, Deserialize)]
pub struct PlayerStatusUpdate {
    pub account_id: Uuid,
    // pub new_status: Option<MatchMakingStatus>,
    pub status: PlayerStatus,
    pub game_type: Option<GameType>,
    pub ranked: Option<bool>,
    pub game_id: Option<String>,
    pub expire_in_seconds: Option<i32>,
}

// impl PlayerStatusUpdate {
//     pub fn game_type_to_str(&self) -> &'static str {
//         self.game_type.as_ref().map(|s| s.to_str()).unwrap_or("none")
//     }

//     pub fn ranked_to_str(&self) -> &'static str {
//         self.ranked.as_ref().map(|s| &s.to_string()).unwrap_or("none")
//     }
// }

// pub async fn set_player_statuses(state: AppState, status_updates: &[PlayerStatusUpdate]) -> Result<(), BunnyChessApiError> {
//     for update in status_updates {
//         let account_key = get_account_status_key(&update.account_id);

//         //services::redis::hset(&state.redis, &account_key).await?
//     }
//     Ok(())
// }

/*
async getPlayerStatus(accountId: string): Promise<MatchmakingStatus> {
    const result = await this.redis.hgetall(this.getAccountStatusKey(accountId));
    return isEmpty(result)
      ? { status: PlayerStatus.Undefined }
      : {
          status: result.status as PlayerStatus,
          gameType: result.gameType as GameType,
          ranked: result.ranked === 'true',
          gameId: result.gameId,
        };
  }
*/


// fn get_account_status_key(account_id: &Uuid) -> String {
//     format!("matchmaking:account:{}:status", account_id)
// }
