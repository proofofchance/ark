use diesel::prelude::Queryable;

use ark_utils::strings;

use serde::{Deserialize, Serialize};

use crate::Chain;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "ongoing")]
    // Ongoing will transition straight to completed because our DApp will resiliently complete the game if it is unresolved or completed.
    // We will handle expired ststus statelessly
    Ongoing,
    #[serde(rename = "expired")]
    Expired,
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
    pub fn is_in_play_phase(&self) -> bool {
        !self.is_expired() && self.play_count < self.max_play_count
    }
    pub fn is_completed(&self) -> bool {
        self.get_status() == GameStatus::Completed
    }
    pub fn get_status(&self) -> GameStatus {
        if self.is_completed {
            GameStatus::Completed
        } else if self.is_expired() {
            GameStatus::Expired
        } else {
            GameStatus::Ongoing
        }
    }
    pub fn is_expired(&self) -> bool {
        let now = chrono::offset::Utc::now().timestamp();

        self.expiry_timestamp <= now
    }
    pub fn get_wager_ether_unit(&self) -> f64 {
        let wager = strings::truncate_string(&self.wager, 10);
        let wager_int: f64 = wager.parse().unwrap();

        wager_int / (10 as f64).powf(8.0)
    }
    pub fn get_chain_id(&self) -> i32 {
        match self.chain_id.into() {
            Chain::Local => Chain::Ethereum as i32,
            Chain::LocalAlt => Chain::Ethereum as i32,
            _any_other_chain => self.chain_id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_plays)]
pub struct GamePlay {
    pub id: i32,
    pub game_id: i64,
    pub coin_side: bool,
    pub player_address: String,
    pub play_hash: String,
}

#[derive(Debug, Deserialize)]
pub enum GameField {
    Id,
    MaxPlayCount,
    ExpiryTimestamp,
    BlockNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_play_proofs)]
pub struct GamePlayProof {
    pub id: i64,
    pub game_id: i64,
    pub game_play_id: i32,
    pub player_address: String,
    pub play_proof: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_activities)]
pub struct GameActivity {
    pub id: i64,
    pub game_id: i64,
    pub trigger_public_address: String,
    pub kind: String,
    pub data: serde_json::Value,
    pub block_timestamp: i64,
    pub transaction_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameActivityKind {
    #[serde(rename = "game_created")]
    GameCreated,
    #[serde(rename = "game_play_created")]
    GamePlayCreated,
    #[serde(rename = "game_play_proof_created")]
    GamePlayProofCreated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GamePlayCreatedActivityData {
    pub coin_side: bool,
    pub play_hash: String,
}
