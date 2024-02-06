use ark_db::schema::coinflip_chain_currencies;

use diesel::prelude::{Insertable, Queryable};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone)]
pub enum Chain {
    Arbitrum = 42161,
    Avalanche = 43114,
    Binance = 56,
    Ethereum = 1,
    Local = 31337,
    LocalAlt = 1337,
    Optimism = 10,
    Polygon = 137,
    Sepolia = 11155111,
}

impl From<u64> for Chain {
    fn from(value: u64) -> Self {
        match value {
            42161 => Chain::Arbitrum,
            43114 => Chain::Avalanche,
            56 => Chain::Binance,
            1 => Chain::Ethereum,
            31337 => Chain::Local,
            1337 => Chain::LocalAlt,
            10 => Chain::Optimism,
            137 => Chain::Polygon,
            11155111 => Chain::Sepolia,
            _else => unimplemented!("Chain id not supported"),
        }
    }
}

impl From<chaindexing::Chain> for Chain {
    fn from(value: chaindexing::Chain) -> Self {
        match value {
            chaindexing::Chain::Arbitrum => Chain::Arbitrum,
            chaindexing::Chain::Avalanche => Chain::Avalanche,
            chaindexing::Chain::BinanceSmartChain => Chain::Binance,
            chaindexing::Chain::Mainnet => Chain::Ethereum,
            chaindexing::Chain::Dev => Chain::Local,
            chaindexing::Chain::Optimism => Chain::Optimism,
            chaindexing::Chain::Polygon => Chain::Polygon,
            chaindexing::Chain::Sepolia => Chain::Sepolia,
            _ => unimplemented!("Unsupported chain"),
        }
    }
}

impl Chain {
    pub fn get_currency_symbol(&self) -> &'static str {
        match self {
            Chain::Arbitrum => "ARB",
            Chain::Avalanche => "AVAX",
            Chain::Binance => "Binance",
            Chain::Ethereum => "ETH",
            Chain::Local => "LocalETH",
            Chain::LocalAlt => "LocalETH",
            Chain::Optimism => "OP",
            Chain::Polygon => "MATIC",
            Chain::Sepolia => "SepoliaETH",
        }
    }

    pub fn from_currency_symbol(currency_symbol: &str) -> Chain {
        match currency_symbol {
            "ARB" => Chain::Arbitrum,
            "AVAX" => Chain::Avalanche,
            "Binance" => Chain::Binance,
            "ETH" => Chain::Ethereum,
            "LocalETH" => Chain::Local,
            "OP" => Chain::Optimism,
            "MATIC" => Chain::Polygon,
            "SepoliaETH" => Chain::Sepolia,
            _ => unimplemented!("Invalid currency symbol"),
        }
    }
}

pub fn get_test_nets() -> Vec<Chain> {
    vec![Chain::Local, Chain::LocalAlt, Chain::Sepolia]
}

#[derive(Clone, Debug, Insertable)]
#[diesel(table_name = coinflip_chain_currencies)]
pub struct UnsavedChainCurrency {
    chain_id: i64,
    currency_symbol: String,
    unit_usd_price: String,
}

impl UnsavedChainCurrency {
    pub fn new(chain: Chain, currency_symbol: &str, unit_usd_price: f32) -> UnsavedChainCurrency {
        UnsavedChainCurrency {
            chain_id: chain as i64,
            currency_symbol: currency_symbol.to_string(),
            unit_usd_price: unit_usd_price.to_string(),
        }
    }
}

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = coinflip_chain_currencies)]
pub struct ChainCurrency {
    _id: i32,
    pub chain_id: i64,
    pub currency_symbol: String,
    unit_usd_price: String,
}

impl ChainCurrency {
    pub fn convert_to_usd(&self, value: f64) -> f64 {
        let unit_usd_price: f64 = self.unit_usd_price.parse().unwrap();

        value * unit_usd_price
    }
}
