use diesel::prelude::Queryable;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
#[diesel(table_name = coinflip_games)]
pub struct Game {
    id: i64,
    max_play_count: i32,
    expiry_timestamp: i64,
    creator_address: String,
    block_number: i64,
}

pub enum GameField {
    Id,
    MaxPlayCount,
    ExpiryTimestamp,
    BlockNumber,
}
