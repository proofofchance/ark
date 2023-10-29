use chaindexing::{ContractState, ContractStateMigrations};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u64,
    pub max_play_count: u32,
    pub expiry_timestamp: u64,
    pub creator_address: String,
    pub wager: String,
    pub play_count: u32,
    pub is_completed: bool,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
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
                is_completed BOOLEAN NOT NULL
            )",
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamePlay {
    pub id: u16,
    pub game_id: u64,
    pub coin_side: u8,
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
                play_hash TEXT NOT NULL,
            )",
        ]
    }
}
