use chaindexing::{ContractState, ContractStateMigrations};

use serde::{Deserialize, Serialize};

use coinflip::CoinSides;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u64,
    pub number_of_players: u32,
    pub expiry_timestamp: u64,
    pub creator_address: String,
    pub wager: String,
    pub play_count: u32,
    pub head_play_count: u32,
    pub tail_play_count: u32,
    pub unavailable_coin_side: Option<u8>,
    pub winner_address: Option<String>,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
    }
}

impl Game {
    pub fn get_unavailable_coin_side(&self, coin_sides: &Vec<u8>) -> Option<u8> {
        self.unavailable_coin_side.or_else(|| {
            if CoinSides::is_all_same(coin_sides) && self.has_one_play_left(coin_sides) {
                coin_sides.first().cloned()
            } else {
                None
            }
        })
    }
    fn has_one_play_left(&self, coin_sides: &Vec<u8>) -> bool {
        (self.number_of_players - 1) as usize == coin_sides.len()
    }
}

pub struct GameMigrations;

impl ContractStateMigrations for GameMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_games (
                id BIGINT NOT NULL,
                number_of_players INTEGER NOT NULL,
                expiry_timestamp BIGINT NOT NULL,
                creator_address VARCHAR NOT NULL,
                wager VARCHAR NOT NULL,
                play_count INTEGER NOT NULL,
                head_play_count INTEGER NOT NULL,
                tail_play_count INTEGER NOT NULL,
                unavailable_coin_side INTEGER,
                outcome INTEGER,
                amount_for_each_winner VARCHAR,
                completed_at BIGINT,
                winner_address VARCHAR,
                refunded_amount_per_player VARCHAR,
                refunded_at BIGINT,
                chain_agnostic_index BIGSERIAL
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
    pub proof_of_chance: String,
    pub status: String,
    pub chance_and_salt: Option<String>,
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
                coin_side INTEGER NOT NULL,
                player_address VARCHAR NOT NULL,
                proof_of_chance VARCHAR NOT NULL,
                status VARCHAR NOT NULL,
                chance_and_salt VARCHAR,
            )",
        ]
    }
}
