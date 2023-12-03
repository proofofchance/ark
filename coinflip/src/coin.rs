use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CoinSide {
    Head,
    Tail,
}

impl CoinSide {
    pub fn from_u8_to_bool(coin_side: u8) -> bool {
        match coin_side {
            0 => false,
            1 => true,
            _ => panic!("Invalid coin side"),
        }
    }

    pub fn is_head_bool(coin_side: bool) -> bool {
        !coin_side
    }

    pub fn is_tail_bool(coin_side: bool) -> bool {
        coin_side
    }

    pub fn is_head_u8(coin_side: u8) -> bool {
        coin_side == 0
    }

    pub fn is_tail_u8(coin_side: u8) -> bool {
        coin_side == 1
    }
}

impl From<usize> for CoinSide {
    fn from(value: usize) -> Self {
        match value {
            0 => CoinSide::Head,
            1 => CoinSide::Tail,
            _ => panic!("Invalid coin side"),
        }
    }
}

impl From<bool> for CoinSide {
    fn from(value: bool) -> Self {
        match value {
            false => CoinSide::Head,
            true => CoinSide::Tail,
        }
    }
}

impl Into<bool> for CoinSide {
    fn into(self) -> bool {
        match self {
            CoinSide::Head => false,
            CoinSide::Tail => true,
        }
    }
}

pub struct CoinSides;

impl CoinSides {
    pub fn is_all_same_bool(coin_sides: &Vec<bool>) -> bool {
        if let Some(first_coin_side) = coin_sides.first() {
            coin_sides.iter().all(|coin_side| coin_side == first_coin_side)
        } else {
            false
        }
    }

    pub fn is_all_same_u8(coin_sides: &Vec<u8>) -> bool {
        if let Some(first_coin_side) = coin_sides.first() {
            coin_sides.iter().all(|coin_side| coin_side == first_coin_side)
        } else {
            false
        }
    }
}
