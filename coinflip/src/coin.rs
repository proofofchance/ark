use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CoinSide {
    Head,
    Tail,
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

pub struct CoinSides;

impl CoinSides {
    pub fn is_all_same_u8(coin_sides: &Vec<u8>) -> bool {
        if let Some(first_coin_side) = coin_sides.first() {
            coin_sides.iter().all(|coin_side| coin_side == first_coin_side)
        } else {
            false
        }
    }
}
