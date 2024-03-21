use ark_db::schema::ark_chain_currencies;

use diesel::prelude::{Insertable, Queryable};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone)]
pub enum ChainId {
    Arbitrum = 42161,
    Avalanche = 43114,
    Ethereum = 1,
    Local = 31337,
    LocalAlt = 1337,
    Optimism = 10,
    Polygon = 137,
    Sepolia = 11155111,
}

impl From<u64> for ChainId {
    fn from(value: u64) -> Self {
        match value {
            42161 => ChainId::Arbitrum,
            43114 => ChainId::Avalanche,
            1 => ChainId::Ethereum,
            31337 => ChainId::Local,
            1337 => ChainId::LocalAlt,
            10 => ChainId::Optimism,
            137 => ChainId::Polygon,
            11155111 => ChainId::Sepolia,
            _else => unimplemented!("Chain id not supported"),
        }
    }
}

impl From<chaindexing::ChainId> for ChainId {
    fn from(value: chaindexing::ChainId) -> Self {
        match value {
            chaindexing::ChainId::Arbitrum => ChainId::Arbitrum,
            chaindexing::ChainId::Avalanche => ChainId::Avalanche,
            chaindexing::ChainId::Mainnet => ChainId::Ethereum,
            chaindexing::ChainId::Dev => ChainId::Local,
            chaindexing::ChainId::Optimism => ChainId::Optimism,
            chaindexing::ChainId::Polygon => ChainId::Polygon,
            chaindexing::ChainId::Sepolia => ChainId::Sepolia,
            _ => unimplemented!("Unsupported chain"),
        }
    }
}

impl ChainId {
    pub fn get_currency_symbol(&self) -> &'static str {
        match self {
            ChainId::Arbitrum => "ARB",
            ChainId::Avalanche => "AVAX",
            ChainId::Ethereum => "ETH",
            ChainId::Local | ChainId::LocalAlt => "LocalETH",
            ChainId::Optimism => "OP",
            ChainId::Polygon => "MATIC",
            ChainId::Sepolia => "SepoliaETH",
        }
    }

    pub fn get_contract_address(&self, contract_name: &str) -> String {
        dotenvy::dotenv().ok();

        let contract_name = &contract_name.to_uppercase();
        let chain_env_namespace = self.get_env_namespace();
        let env_var = &format!("{chain_env_namespace}_{contract_name}_CONTRACT_ADDRESS");

        std::env::var(env_var).expect(&format!("{env_var} must be set"))
    }

    pub fn get_start_block_number(&self, contract_name: &str) -> i64 {
        dotenvy::dotenv().ok();

        let contract_name = &contract_name.to_uppercase();
        let chain_env_namespace = self.get_env_namespace();
        let env_var = &format!("{chain_env_namespace}_{contract_name}_START_BLOCK_NUMBER");

        std::env::var(env_var)
            .expect(&format!("{env_var} must be set"))
            .parse()
            .unwrap()
    }

    fn get_env_namespace(&self) -> &'static str {
        match self {
            ChainId::Arbitrum => "ARBITRUM",
            ChainId::Avalanche => "AVALANCHE",
            ChainId::Ethereum => "ETHEREUM",
            ChainId::Local | ChainId::LocalAlt => "LOCAL",
            ChainId::Optimism => "OPTIMISM",
            ChainId::Polygon => "POLYGON",
            ChainId::Sepolia => "SEPOLIA",
        }
    }

    pub fn from_currency_symbol(currency_symbol: &str) -> ChainId {
        match currency_symbol {
            "ARB" => ChainId::Arbitrum,
            "AVAX" => ChainId::Avalanche,
            "ETH" => ChainId::Ethereum,
            "LocalETH" => ChainId::Local,
            "OP" => ChainId::Optimism,
            "MATIC" => ChainId::Polygon,
            "SepoliaETH" => ChainId::Sepolia,
            _ => unimplemented!("Invalid currency symbol"),
        }
    }
}

pub fn get_test_nets() -> Vec<ChainId> {
    vec![ChainId::Local, ChainId::LocalAlt, ChainId::Sepolia]
}

#[derive(Clone, Debug, Insertable)]
#[diesel(table_name = ark_chain_currencies)]
pub struct UnsavedChainCurrency {
    chain_id: i64,
    currency_symbol: String,
    unit_usd_price: String,
}

impl UnsavedChainCurrency {
    pub fn new(
        chain_id: ChainId,
        currency_symbol: &str,
        unit_usd_price: f32,
    ) -> UnsavedChainCurrency {
        UnsavedChainCurrency {
            chain_id: chain_id as i64,
            currency_symbol: currency_symbol.to_string(),
            unit_usd_price: unit_usd_price.to_string(),
        }
    }
}

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = ark_chain_currencies)]
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
