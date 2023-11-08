use diesel::prelude::Queryable;

use ark_utils::strings;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "ongoing")]
    // Ongoing will transition straight to completed because our DApp will resiliently complete the game if it is unresolved or completed.
    // We will handle expired ststus statelessly
    Ongoing,
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
    pub chain_id: i32,
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
            GameStatus::Ongoing
        }
    }
    pub fn get_wager_ether_unit(&self) -> f64 {
        let wager = strings::truncate_string(&self.wager, 10);
        let wager_int: f64 = wager.parse().unwrap();

        wager_int / (10 as f64).powf(8.0)
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
