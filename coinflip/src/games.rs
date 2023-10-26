use diesel::prelude::Queryable;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_games)]
pub struct Game {
    id: i64,
    max_play_count: i32,
    expiry_timestamp: i64,
    creator_address: String,
    block_number: i64,
}

#[derive(Debug, Deserialize)]
pub enum GameField {
    Id,
    MaxPlayCount,
    ExpiryTimestamp,
    BlockNumber,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameStatus {
    Available,
    Ongoing,
    Completed,
}

impl From<&str> for GameStatus {
    fn from(value: &str) -> Self {
        match value {
            "available" => GameStatus::Available,
            "ongoing" => GameStatus::Ongoing,
            "completed" => GameStatus::Completed,
            _ => unreachable!("Invalid GameStatus found!"),
        }
    }
}
