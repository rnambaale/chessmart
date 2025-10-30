use std::str::FromStr;

use redis::ToRedisArgs;
use serde::{Deserialize, Serialize};

use crate::error::BunnyChessApiError;

#[derive(Serialize, Deserialize, Clone, Copy)]
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

impl FromStr for GameType {
    type Err = BunnyChessApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Rapid10_0" => Ok(GameType::Rapid10_0),
            "Blitz5_3" => Ok(GameType::Blitz5_3),
            "Blitz5_0" => Ok(GameType::Blitz5_0),
            "Blitz3_2" => Ok(GameType::Blitz3_2),
            "Blitz3_0" => Ok(GameType::Blitz3_0),
            "Bullet1_0" => Ok(GameType::Bullet1_0),
            _ => Err(BunnyChessApiError::UnknownGameTypeError(s.into())),
        }
    }
}

impl ToRedisArgs for GameType {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        // Use string representation
        out.write_arg(self.to_str().as_bytes());

        // Or use numeric representation:
        // out.write_arg_fmt(self.as_i32());
    }
}
