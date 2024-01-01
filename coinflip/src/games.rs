use ark_db::schema::coinflip_game_activities;
use diesel::prelude::{Insertable, Queryable};

use ark_utils::strings;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::Chain;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "ongoing")]
    // Ongoing will transition straight to completed because our DApp will resiliently complete the game if it is unresolved or completed.
    // We will handle expired ststus statelessly
    Ongoing,
    #[serde(rename = "awaiting_proofs_upload")]
    AwaitingProofsUpload,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "completed")]
    Completed,
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_games)]
pub struct Game {
    pub id: i64,
    pub chain_id: i64,
    pub max_play_count: i32,
    pub expiry_timestamp: i64,
    pub creator_address: String,
    pub block_number: i64,
    pub wager: String,
    pub play_count: i32,
    // TODO: Listen to expired/winners_unresolved events and then resolve, and then mark as complete
    pub is_completed: bool,
    pub unavailable_coin_side: Option<i32>,
    pub proofs_uploaded_at: Option<i64>,
}

impl Game {
    pub fn get_players_left(&self) -> u32 {
        (self.max_play_count - self.play_count) as u32
    }
    pub fn is_ongoing(&self) -> bool {
        self.get_status() == GameStatus::Ongoing
    }
    pub fn is_awaiting_proofs_upload(&self) -> bool {
        self.get_status() == GameStatus::AwaitingProofsUpload
    }
    pub fn is_completed(&self) -> bool {
        self.get_status() == GameStatus::Completed
    }
    pub fn is_expired(&self) -> bool {
        self.get_status() == GameStatus::Expired
    }
    pub fn get_status(&self) -> GameStatus {
        let now = chrono::offset::Utc::now().timestamp();

        if self.expiry_timestamp <= now {
            GameStatus::Expired
        } else if self.play_count < self.max_play_count {
            GameStatus::Ongoing
        } else if self.play_count == self.max_play_count {
            GameStatus::AwaitingProofsUpload
        } else {
            panic!("TODO: Unknown game status");
        }
    }

    pub fn get_wager_ether_unit(&self) -> f64 {
        let wager = strings::truncate_string(&self.wager, 10);
        let wager_int: f64 = wager.parse().unwrap();

        wager_int / (10 as f64).powf(8.0)
    }
    pub fn get_chain_id(&self) -> i64 {
        match self.chain_id.into() {
            Chain::Local => Chain::Ethereum as i64,
            Chain::LocalAlt => Chain::Ethereum as i64,
            _any_other_chain => self.chain_id,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_plays)]
pub struct GamePlay {
    pub id: i32,
    pub game_id: i64,
    pub chain_id: i64,
    pub coin_side: i32,
    pub player_address: String,
    pub play_hash: String,
    pub play_proof: Option<String>,
}

impl GamePlay {
    pub fn is_play_proof(&self, play_proof: &str) -> bool {
        self.play_hash == hash_proof(play_proof)
    }
}

fn hash_proof(play_proof: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(play_proof);
    hex::encode(hasher.finalize())
}

#[derive(Debug, Deserialize)]
pub enum GameField {
    Id,
    MaxPlayCount,
    ExpiryTimestamp,
    BlockNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameActivityKind {
    #[serde(rename = "game_created")]
    GameCreated,
    #[serde(rename = "game_play_created")]
    GamePlayCreated,
    #[serde(rename = "game_play_proof_created")]
    GamePlayProofCreated,
    #[serde(rename = "game_expired")]
    GameExpired,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = coinflip_game_activities)]
pub struct UnsavedGameActivity {
    pub game_id: i64,
    pub chain_id: i64,
    pub trigger_public_address: String,
    pub kind: String,
    pub data: Option<serde_json::Value>,
    pub block_timestamp: Option<i64>,
    pub transaction_hash: Option<String>,
}

impl UnsavedGameActivity {
    pub fn new_game_created(
        game_id: u64,
        chain_id: i64,
        trigger_public_address: String,
        block_timestamp: i64,
        transaction_hash: String,
    ) -> Self {
        UnsavedGameActivity {
            game_id: game_id as i64,
            chain_id,
            block_timestamp: Some(block_timestamp),
            trigger_public_address: trigger_public_address.to_lowercase(),
            kind: "game_created".to_string(),
            data: None,
            transaction_hash: Some(transaction_hash.to_lowercase()),
        }
    }
    pub fn new_game_play_created(
        game_id: u64,
        chain_id: i64,
        trigger_public_address: String,
        block_timestamp: i64,
        transaction_hash: String,
        coin_side: u8,
        play_hash: String,
    ) -> Self {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct GamePlayCreatedActivityData {
            pub coin_side: u8,
            pub play_hash: String,
        }

        UnsavedGameActivity {
            game_id: game_id as i64,
            chain_id,
            block_timestamp: Some(block_timestamp),
            trigger_public_address: trigger_public_address.to_lowercase(),
            kind: "game_play_created".to_string(),
            data: Some(
                serde_json::to_value(GamePlayCreatedActivityData {
                    coin_side,
                    play_hash,
                })
                .unwrap(),
            ),
            transaction_hash: Some(transaction_hash.to_lowercase()),
        }
    }
    pub fn new_proof_created(game_id: u64, chain_id: i64, trigger_public_address: String) -> Self {
        UnsavedGameActivity {
            game_id: game_id as i64,
            chain_id,
            trigger_public_address: trigger_public_address.to_lowercase(),
            kind: "game_play_proof_created".to_string(),
            data: None,
            block_timestamp: None,
            transaction_hash: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_game_activities)]
pub struct GameActivity {
    pub id: i64,
    pub game_id: i64,
    pub chain_id: i64,
    pub trigger_public_address: String,
    pub kind: String,
    pub data: serde_json::Value,
    pub block_timestamp: Option<i64>,
    pub transaction_hash: Option<String>,
}

impl GameActivity {
    pub fn new_expired(game_id: i64, chain_id: i64, expiry_timestamp: i64) -> Self {
        Self {
            id: 0,
            game_id,
            chain_id,
            trigger_public_address: "0x".to_string(),
            kind: "game_expired".to_string(),
            data: serde_json::Value::Null,
            block_timestamp: Some(expiry_timestamp),
            transaction_hash: Some("0x".to_string()),
        }
    }
}
