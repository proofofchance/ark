use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq)]
pub enum Chain {
    Arbitrum = 42161,
    Avalanche = 43114,
    Bnb = 56,
    Ethereum = 1,
    Local = 31337,
    LocalAlt = 1337,
    Optimism = 10,
    Polygon = 137,
    SepoliaTestNet = 11155111,
}

impl Chain {
    pub fn get_currency(&self) -> &'static str {
        match self {
            Chain::Arbitrum => "ARB",
            Chain::Avalanche => "AVAX",
            Chain::Bnb => "BNB",
            Chain::Ethereum => "ETH",
            Chain::Local => "LocalETH",
            Chain::LocalAlt => "LocalETH",
            Chain::Optimism => "OP",
            Chain::Polygon => "MATIC",
            Chain::SepoliaTestNet => "SepoliaETH",
        }
    }
}

pub fn get_test_nets() -> Vec<Chain> {
    vec![Chain::Local, Chain::LocalAlt, Chain::SepoliaTestNet]
}
