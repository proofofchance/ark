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
