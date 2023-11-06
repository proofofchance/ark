use ark_db::schema::coinflip_chain_currencies;

use diesel::prelude::{Insertable, Queryable};
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
    pub fn get_currency_symbol(&self) -> &'static str {
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

    pub fn from_currency_symbol(currency_symbol: &str) -> Chain {
        match currency_symbol {
            "ARB" => Chain::Arbitrum,
            "AVAX" => Chain::Avalanche,
            "BNB" => Chain::Bnb,
            "ETH" => Chain::Ethereum,
            "LocalETH" => Chain::Local,
            "OP" => Chain::Optimism,
            "MATIC" => Chain::Polygon,
            "SepoliaETH" => Chain::SepoliaTestNet,
            _ => panic!("Invalid currency symbol"),
        }
    }
}

pub fn get_test_nets() -> Vec<Chain> {
    vec![Chain::Local, Chain::LocalAlt, Chain::SepoliaTestNet]
}

#[derive(Clone, Debug, Insertable)]
#[diesel(table_name = coinflip_chain_currencies)]
pub struct UnsavedChainCurrency {
    chain_id: i32,
    currency_symbol: String,
    unit_usd_price: String,
}

impl UnsavedChainCurrency {
    pub fn new(chain: Chain, currency_symbol: &str, unit_usd_price: f32) -> UnsavedChainCurrency {
        UnsavedChainCurrency {
            chain_id: chain as i32,
            currency_symbol: currency_symbol.to_string(),
            unit_usd_price: unit_usd_price.to_string(),
        }
    }
}

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = coinflip_chain_currencies)]
pub struct ChainCurrency {
    id: i32,
    pub chain_id: i32,
    pub currency_symbol: String,
    unit_usd_price: String,
}

impl ChainCurrency {
    pub fn convert_to_usd(&self, value: f64) -> f64 {
        let unit_usd_price: f64 = self.unit_usd_price.parse().unwrap();

        value * unit_usd_price
    }
}
