use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use shakmaty::{Chess, Color, Position};
use shared::primitives::GameType;

// const MAX_MOVES: u64 = 300;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIds {
    pub w: String,
    pub b: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameRules {
    time_limit_ms: u64,
    time_increase_per_turn_ms: u64,
}

impl GameRules {
    pub fn new(time_limit_ms: u64, time_increase_per_turn_ms: u64) -> Self {
        Self {
            time_limit_ms,
            time_increase_per_turn_ms,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameClocks {
    w: u64,
    b: u64,
    start_timestamp: DateTime<Utc>,
    last_move_timestamp: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct ChessGame {
    pub id: String,
    pub chess: Chess,
    pub game_type: GameType,
    pub account_ids: AccountIds,
    pub metadata: String,
    pub game_rules: GameRules,
    pub game_clocks: GameClocks,
    pub resigned_color: Option<ColorWrapper>,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorWrapper(Color);

impl From<Color> for ColorWrapper {
    fn from(color: Color) -> Self {
        ColorWrapper(color)
    }
}

impl From<ColorWrapper> for Color {
    fn from(wrapper: ColorWrapper) -> Self {
        wrapper.0
    }
}

impl Serialize for ColorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as the numeric value
        serializer.serialize_u8(self.0 as u8)

        // Or serialize as string:
        // match self.0 {
        //     Color::Black => serializer.serialize_str("black"),
        //     Color::White => serializer.serialize_str("white"),
        // }
    }
}

impl<'de> Deserialize<'de> for ColorWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // For numeric deserialization:
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(ColorWrapper(Color::Black)),
            1 => Ok(ColorWrapper(Color::White)),
            _ => Err(serde::de::Error::custom("Invalid color value")),
        }

        // Or for string deserialization:
        // let s = String::deserialize(deserializer)?;
        // match s.as_str() {
        //     "black" | "0" => Ok(ColorWrapper(Color::Black)),
        //     "white" | "1" => Ok(ColorWrapper(Color::White)),
        //     _ => Err(serde::de::Error::custom("Invalid color value")),
        // }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRepr {
    pub id: String,
    pub pgn: String,
    pub game_type: String,
    pub account_ids: AccountIds,
    pub metadata: String,
    pub game_rules: GameRules,
    pub game_clocks: GameClocks,
    pub resigned_color: Option<ColorWrapper>,
    pub seq: u64,
}

impl ChessGame {
    pub fn new(
        id: String,
        chess: Chess,
        game_type: GameType,
        account_ids: AccountIds,
        metadata: String,
        game_rules: GameRules,
        game_clocks: GameClocks,
    ) -> Self {
        Self {
            id,
            chess,
            game_type,
            account_ids,
            metadata,
            game_rules,
            game_clocks,
            resigned_color: None,
        }
    }

    pub fn from_scratch(
        game_type: GameType,
        account_ids: AccountIds,
        metadata: String,
        game_rules: GameRules
    ) -> Self {
        let GameRules { time_limit_ms, .. } = game_rules.clone();

        Self::new(
            uuid::Uuid::new_v4().to_string(),
            Chess::new(),
            game_type,
            account_ids,
            metadata,
            game_rules,
            GameClocks {
                w: time_limit_ms,
                b: time_limit_ms,
                start_timestamp: Utc::now(),
                last_move_timestamp: None,
            }
        )
    }

    // Getters
    // pub fn id(&self) -> &str {
    //     &self.id
    // }

    // pub fn chess(&self) -> &Chess {
    //     &self.chess
    // }

    // pub fn account_ids(&self) -> &AccountIds {
    //     &self.account_ids
    // }

    // pub fn game_rules(&self) -> &GameRules {
    //     &self.game_rules
    // }

    // pub fn game_clocks(&self) -> &GameClocks {
    //     &self.game_clocks
    // }

    // pub fn resigned_color(&self) -> Option<ColorWrapper> {
    //     self.resigned_color
    // }

    pub fn seq(&self) -> u64 {
        self.chess.fullmoves().get() as u64
    }

    // Setters
    // pub fn set_resigned_color(&mut self, color: ColorWrapper) {
    //     self.resigned_color = Some(color);
    // }

    // pub fn update_game_clocks(&mut self, new_clocks: GameClocks) {
    //     self.game_clocks = new_clocks;
    // }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // Always update the clocks before serializing the game
        // self.update_clock();

        let json_repr = JsonRepr {
            id: self.id.clone(),
            pgn: self.get_pgn(),
            game_type: self.game_type.to_str().to_owned(),
            account_ids: self.account_ids.clone(),
            metadata: self.metadata.clone(),
            game_rules: self.game_rules.clone(),
            game_clocks: self.game_clocks.clone(),
            resigned_color: self.resigned_color,
            seq: self.seq(),
        };

        serde_json::to_string(&json_repr)
    }

    pub fn to_string(&self) -> String {
        self.to_json().unwrap_or_else(|_| "{}".to_string())
    }

    // fn get_pgn(&self) -> String {
    //     // Use shakmaty's PGN export functionality
    //     // The exact method depends on how you're using shakmaty
    //     // This might involve using shakmaty::pgn::PgnBuilder or similar
    //     // "".to_string() // Placeholder - implement based on your shakmaty usage
    //     todo!()
    // }

    fn get_pgn(&self) -> String {
        // Example implementation - adjust based on your needs
        // let mut builder = shakmaty::pgn::PgnBuilder::new();
        // Add moves, headers, etc. to the builder
        // builder.header("White", &self.account_ids.white);
        // builder.header("Black", &self.account_ids.black);
        // ...
        // builder.finish()
        todo!()
    }

    // fn update_clock(&mut self) {
    //     // Implementation would go here
    // }
}

impl std::fmt::Display for ChessGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct CreateGameDto {
    pub account_id0: String,
    pub account_id1: String,
    pub game_type: shared::primitives::GameType,
    pub metadata: String,
}

fn minutes(minutes: u64) -> u64 {
    minutes * 60 * 1000
}

fn seconds(seconds: u64) -> u64 {
    seconds * 1000
}

pub fn get_game_rules(game_type: GameType) -> GameRules {
    match game_type {
        GameType::Rapid10_0 => GameRules::new(
            minutes(10),
            0
        ),
        GameType::Blitz5_3 => GameRules::new(
            minutes(5),
            seconds(3)
        ),
        GameType::Blitz5_0 => GameRules::new(
            minutes(5),
            0
        ),
        GameType::Blitz3_2 => GameRules::new(
            minutes(3),
            seconds(2)
        ),
        GameType::Blitz3_0 => GameRules::new(
            minutes(3),
            0
        ),
        GameType::Bullet1_0 => GameRules::new(
            minutes(1),
            0
        ),
    }
}
