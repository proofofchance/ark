use chaindexing::{ContractState, ContractStateMigrations};

use serde::{Deserialize, Serialize};

use crate::coin::CoinSides;

// Index early to allow server have any computing memory
// server should do less work in memory, so cache early!

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u64,
    pub max_play_count: u32,
    pub expiry_timestamp: u64,
    pub creator_address: String,
    pub wager: String,
    pub play_count: u32,
    pub head_play_count: u32,
    pub tail_play_count: u32,
    pub is_completed: bool,
    pub unavailable_coin_side: Option<u8>,
    pub winner_address: Option<String>,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
    }
}

impl Game {
    pub fn get_unavailable_coin_side(&self, game_plays: &Vec<u8>) -> Option<u8> {
        self.unavailable_coin_side.or_else(|| {
            if CoinSides::is_all_same_u8(game_plays) && self.has_one_play_left(game_plays) {
                game_plays.first().cloned()
            } else {
                None
            }
        })
    }
    fn has_one_play_left(&self, game_plays: &Vec<u8>) -> bool {
        (self.max_play_count - 1) as usize == game_plays.len()
    }
}

pub struct GamesMigrations;

impl ContractStateMigrations for GamesMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_games (
                id BIGINT PRIMARY KEY,
                max_play_count INTEGER NOT NULL,
                expiry_timestamp BIGINT NOT NULL,
                creator_address VARCHAR NOT NULL,
                wager TEXT NOT NULL,
                play_count INTEGER NOT NULL,
                head_play_count INTEGER NOT NULL,
                tail_play_count INTEGER NOT NULL,
                is_completed BOOLEAN NOT NULL,
                unavailable_coin_side INTEGER,
                winner_address VARCHAR
            )",
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamePlay {
    pub id: u16,
    pub game_id: u64,
    pub coin_side: u8,
    pub player_address: String,
    pub play_hash: String,
}

impl ContractState for GamePlay {
    fn table_name() -> &'static str {
        "coinflip_game_plays"
    }
}

pub struct GamePlaysMigrations;

impl ContractStateMigrations for GamePlaysMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_game_plays (
                id INTEGER NOT NULL,
                game_id BIGINT NOT NULL,
                coin_side INTEGER NOT NULL,
                player_address TEXT NOT NULL,
                play_hash TEXT NOT NULL,
            )",
        ]
    }
}
