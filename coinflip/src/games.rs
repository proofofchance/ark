use diesel::prelude::Queryable;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "available")]
    // Available will transition straight to completed because our DApp will resiliently complete the game if it is unresolved or expired
    Available,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_games)]
pub struct Game {
    pub id: i64,
    pub max_play_count: i32,
    pub expiry_timestamp: i64,
    pub creator_address: String,
    pub block_number: i64,
    pub wager: String,
    pub play_count: i32,
    // TODO: Listen to expired/winners_unresolved events and then resolve, and then mark as complete
    pub is_completed: bool,
}

impl Game {
    pub fn get_players_left(&self) -> u32 {
        (self.max_play_count - self.play_count) as u32
    }
    pub fn get_status(&self) -> GameStatus {
        if self.is_completed {
            GameStatus::Completed
        } else {
            GameStatus::Available
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_plays)]
pub struct GamePlay {
    pub id: i32,
    pub game_id: i64,
    pub coin_side: bool,
    pub play_hash: String,
}

#[derive(Debug, Deserialize)]
pub enum GameField {
    Id,
    MaxPlayCount,
    ExpiryTimestamp,
    BlockNumber,
}
