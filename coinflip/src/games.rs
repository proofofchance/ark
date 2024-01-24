use ark_db::schema::coinflip_game_activities;
use diesel::prelude::{Insertable, Queryable};

use ark_utils::ethers::convert_wei_to_ether;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use ark_web3::chains::Chain;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    #[serde(rename = "ongoing")]
    // Ongoing will transition straight to completed because our DApp will resiliently complete the game if it is unresolved or completed.
    // We will handle expired ststus statelessly
    Ongoing,
    #[serde(rename = "awaiting_revealed_chances")]
    AwaitingRevealedChances,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "completed")]
    Completed,
}

impl<'a> Into<&'a str> for GameStatus {
    fn into(self) -> &'a str {
        match self {
            GameStatus::Ongoing => "ongoing",
            GameStatus::AwaitingRevealedChances => "awaiting_revealed_chances",
            GameStatus::Expired => "expired",
            GameStatus::Completed => "completed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_games)]
pub struct Game {
    pub id: i64,
    pub chain_id: i64,
    pub number_of_players: i32,
    pub expiry_timestamp: i64,
    pub creator_address: String,
    pub block_number: i64,
    pub wager: String,
    pub play_count: i32,
    pub unavailable_coin_side: Option<i32>,
    pub outcome: Option<i32>,
    pub amount_for_each_winner: Option<String>,
    pub completed_at: Option<i64>,
    pub refunded_amount_per_player: Option<String>,
    pub refunded_at: Option<i64>,
}

impl Game {
    pub fn has_all_chances_uploaded(&self, chance_and_salts_size: usize) -> bool {
        self.number_of_players as usize == chance_and_salts_size
    }
    pub fn get_players_left(&self) -> u32 {
        (self.number_of_players - self.play_count) as u32
    }
    pub fn is_ongoing(&self) -> bool {
        self.get_status() == GameStatus::Ongoing
    }
    pub fn is_awaiting_revealed_chances(&self) -> bool {
        self.get_status() == GameStatus::AwaitingRevealedChances
    }
    pub fn is_completed(&self) -> bool {
        self.get_status() == GameStatus::Completed
    }
    pub fn is_expired(&self) -> bool {
        self.get_status() == GameStatus::Expired
    }
    pub fn get_status(&self) -> GameStatus {
        let now = chrono::offset::Utc::now().timestamp();

        if self.expiry_timestamp <= now && self.completed_at.is_none() {
            GameStatus::Expired
        } else if self.play_count < self.number_of_players {
            GameStatus::Ongoing
        } else if self.completed_at.is_none() && self.play_count == self.number_of_players {
            GameStatus::AwaitingRevealedChances
        } else if self.completed_at.is_some() {
            GameStatus::Completed
        } else {
            panic!("TODO: Unknown game status");
        }
    }

    pub fn get_wager_ether(&self) -> f64 {
        convert_wei_to_ether(&self.wager)
    }
    pub fn get_amount_for_each_winner_ether(&self) -> Option<f64> {
        self.amount_for_each_winner.clone().map(|amt| convert_wei_to_ether(&amt))
    }
    pub fn get_refunded_amount_per_player_ether(&self) -> Option<f64> {
        self.refunded_amount_per_player.clone().map(|amt| convert_wei_to_ether(&amt))
    }

    pub fn get_chain_id(&self) -> i64 {
        match self.chain_id.into() {
            Chain::Local => Chain::Ethereum as i64,
            Chain::LocalAlt => Chain::Ethereum as i64,
            _any_other_chain => self.chain_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GamePlayStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "won")]
    Won,
    #[serde(rename = "lost")]
    Lost,
    #[serde(rename = "expired")]
    Expired,
}

impl Into<String> for GamePlayStatus {
    fn into(self) -> String {
        match self {
            GamePlayStatus::Pending => "pending",
            GamePlayStatus::Won => "won",
            GamePlayStatus::Lost => "lost",
            GamePlayStatus::Expired => "expired",
        }
        .to_string()
    }
}

impl From<String> for GamePlayStatus {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "pending" => GamePlayStatus::Pending,
            "won" => GamePlayStatus::Expired,
            "lost" => GamePlayStatus::Lost,
            "expired" => GamePlayStatus::Expired,
            _ => panic!("Unknown game play status"),
        }
    }
}

impl GamePlayStatus {
    pub fn pending_string() -> String {
        "pending".to_string()
    }
    pub fn won_string() -> String {
        "won".to_string()
    }
    pub fn lost_string() -> String {
        "lost".to_string()
    }
    pub fn expired_string() -> String {
        "expired".to_string()
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
    pub proof_of_chance: String,
    pub chance_and_salt: Option<String>,
    pub status: String,
}

impl GamePlay {
    pub fn is_chance_and_salt(&self, chance_and_salt: &str) -> bool {
        // TODO: Is there a bug here? Unit tests!
        self.proof_of_chance == hash_proof(&Self::get_chance_and_salt_bytes(chance_and_salt))
    }
    pub fn get_chance_and_salt_bytes(chance_and_salt: &str) -> Vec<u8> {
        let chance_and_salt = chance_and_salt.replace("0x", "");
        hex::decode(&chance_and_salt).unwrap()
    }

    pub fn get_status(&self) -> GamePlayStatus {
        match self.status.as_ref() {
            "pending" => GamePlayStatus::Pending,
            "won" => GamePlayStatus::Won,
            "lost" => GamePlayStatus::Lost,
            "expired" => GamePlayStatus::Expired,
            _ => unreachable!("Unknown state"),
        }
    }
}

fn hash_proof(chance_and_salt_bytes: &Vec<u8>) -> String {
    let mut sha256 = Sha256::new();
    sha256.update(chance_and_salt_bytes);
    hex::encode(sha256.finalize())
}

#[derive(Debug, Deserialize)]
pub enum GameField {
    Id,
    NumberOfPlayers,
    ExpiryTimestamp,
    BlockNumber,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameActivityKind {
    #[serde(rename = "game_created")]
    GameCreated,
    #[serde(rename = "game_play_created")]
    GamePlayCreated,
    #[serde(rename = "game_play_chance_revealed")]
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
    pub occurred_at: i64,
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
            occurred_at: block_timestamp,
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
        proof_of_chance: String,
    ) -> Self {
        #[derive(Clone, Debug, Serialize, Deserialize)]
        struct GamePlayCreatedActivityData {
            pub coin_side: u8,
            pub proof_of_chance: String,
        }

        UnsavedGameActivity {
            game_id: game_id as i64,
            chain_id,
            occurred_at: block_timestamp,
            trigger_public_address: trigger_public_address.to_lowercase(),
            kind: "game_play_created".to_string(),
            data: Some(
                serde_json::to_value(GamePlayCreatedActivityData {
                    coin_side,
                    proof_of_chance,
                })
                .unwrap(),
            ),
            transaction_hash: Some(transaction_hash.to_lowercase()),
        }
    }
    pub fn new_chance_revealed(
        game_id: u64,
        chain_id: i64,
        trigger_public_address: String,
    ) -> Self {
        let now = chrono::offset::Utc::now().timestamp();

        UnsavedGameActivity {
            game_id: game_id as i64,
            chain_id,
            trigger_public_address: trigger_public_address.to_lowercase(),
            kind: "game_play_chance_revealed".to_string(),
            data: None,
            occurred_at: now,
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
    pub occurred_at: i64,
    pub transaction_hash: Option<String>,
}

pub struct PlayerAddress;

impl PlayerAddress {
    pub fn do_both_match(address_1: &str, address_2: &str) -> bool {
        address_1.to_lowercase() == address_2.to_lowercase()
    }
}
