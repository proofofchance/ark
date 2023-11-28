use chaindexing::{ContractState, ContractStateMigrations};

use serde::{Deserialize, Serialize};

use crate::{coin::CoinSides, GameActivityKind};

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
    pub unavailable_coin_side: Option<bool>,
    pub winner_address: Option<String>,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
    }
}

impl Game {
    pub fn get_unavailable_coin_side(&self, game_plays: &Vec<bool>) -> Option<bool> {
        self.unavailable_coin_side.or_else(|| {
            if CoinSides::is_all_same_bool(game_plays) && self.has_one_play_left(game_plays) {
                game_plays.first().cloned()
            } else {
                None
            }
        })
    }
    fn has_one_play_left(&self, game_plays: &Vec<bool>) -> bool {
        (self.max_play_count - 1) as usize == game_plays.len()
    }
}

pub struct GameMigrations;

impl ContractStateMigrations for GameMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_games (
                id BIGINT PRIMARY KEY,
                max_play_count INTEGER NOT NULL,
                expiry_timestamp BIGINT NOT NULL,
                creator_address VARCHAR NOT NULL,
                wager VARCHAR NOT NULL,
                play_count INTEGER NOT NULL,
                head_play_count INTEGER NOT NULL,
                tail_play_count INTEGER NOT NULL,
                is_completed BOOLEAN NOT NULL,
                unavailable_coin_side BOOLEAN,
                winner_address VARCHAR
            )",
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamePlay {
    pub id: u16,
    pub game_id: u64,
    pub coin_side: bool,
    pub player_address: String,
    pub play_hash: String,
}

impl ContractState for GamePlay {
    fn table_name() -> &'static str {
        "coinflip_game_plays"
    }
}

pub struct GamePlayMigrations;

impl ContractStateMigrations for GamePlayMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_game_plays (
                id INTEGER NOT NULL,
                game_id BIGINT NOT NULL,
                coin_side BOOLEAN NOT NULL,
                player_address VARCHAR NOT NULL,
                play_hash VARCHAR NOT NULL,
            )",
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamePlayProof {
    pub game_id: u64,
    pub game_play_id: u16,
    pub player_address: String,
    pub play_proof: String,
}

impl ContractState for GamePlayProof {
    fn table_name() -> &'static str {
        "coinflip_game_play_proofs"
    }
}

pub struct GamePlayProofMigrations;

impl ContractStateMigrations for GamePlayProofMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_game_play_proofs (
                id BIGSERIAL PRIMARY KEY,
                game_id BIGINT NOT NULL,
                game_play_id INTEGER NOT NULL,
                player_address VARCHAR NOT NULL,
                play_proof VARCHAR NOT NULL,
            )",
        ]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameActivity {
    pub game_id: u64,
    pub trigger_public_address: String,
    pub kind: GameActivityKind,
    pub data: Option<serde_json::Value>,
    pub block_timestamp: u64,
    pub transaction_hash: String,
}

impl ContractState for GameActivity {
    fn table_name() -> &'static str {
        "coinflip_game_activities"
    }
}

pub struct GameActivityMigrations;

impl ContractStateMigrations for GameActivityMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_game_activities (
                id BIGSERIAL PRIMARY KEY,
                game_id BIGINT NOT NULL,
                trigger_public_address VARCHAR NOT NULL,
                kind VARCHAR NOT NULL,
                data JSON DEFAULT '{}',
                block_timestamp BIGINT NOT NULL,
                transaction_hash VARCHAR NOT NULL
            )",
        ]
    }
}
