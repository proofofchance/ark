use chaindexing::ContractState;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: u64,
    pub max_play_count: u32,
    pub expiry_timestamp: u64,
    pub creator_address: String,
    pub wager: u64,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
    }
}
