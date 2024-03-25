use eyre::Result;
use serde::Deserialize;

use crate::{chains::ChainId, CHAIN_AGNOSTIC_MAX_GAS_PRICE, GWEI};

#[derive(Clone, Debug, Deserialize)]
pub struct GasInfo {
    pub last_block: u64,
    pub safe_gas_price: u64,
    pub propose_gas_price: u64,
    pub fast_gas_price: u64,
    pub usd_price: f64,
}

impl Default for GasInfo {
    fn default() -> Self {
        Self {
            last_block: 0,
            safe_gas_price: CHAIN_AGNOSTIC_MAX_GAS_PRICE,
            propose_gas_price: CHAIN_AGNOSTIC_MAX_GAS_PRICE,
            fast_gas_price: CHAIN_AGNOSTIC_MAX_GAS_PRICE,
            usd_price: 1.0,
        }
    }
}

impl GasInfo {
    pub fn get_fast_price_wei(&self) -> u64 {
        self.fast_gas_price * GWEI
    }
    pub fn get_safe_price_wei(&self) -> u64 {
        self.safe_gas_price * GWEI
    }
}

const POLYSCAN_BASE_API_URL: &'static str = "https://api.polygonscan.com/api";
const ETHERSCAN_BASE_API_URL: &'static str = "https://api.etherscan.com/api";

pub async fn get_gas_info(chain_id: &ChainId) -> Result<GasInfo> {
    match chain_id {
        ChainId::Local | ChainId::LocalAlt => Ok(Default::default()),
        ChainId::Polygon => {
            fetch_gas_info(&get_gas_info_api_url(
                POLYSCAN_BASE_API_URL,
                &get_explorer_api_key(chain_id),
            ))
            .await
        }
        ChainId::Sepolia | ChainId::Ethereum => {
            fetch_gas_info(&get_gas_info_api_url(
                ETHERSCAN_BASE_API_URL,
                &get_explorer_api_key(chain_id),
            ))
            .await
        }
        _ => unimplemented!("Chain:{} GasInfo not implemented", *chain_id as u64),
    }
}

async fn fetch_gas_info(api_url: &str) -> Result<GasInfo> {
    let response = reqwest::get(api_url).await?;

    Ok(response.json::<GasInfo>().await?)
}

fn get_gas_info_api_url(base_api_url: &str, api_key: &str) -> String {
    format!("{base_api_url}?module=gastracker&action=gasoracle&apiKey={api_key}")
}

fn get_explorer_api_key(chain_id: &ChainId) -> String {
    dotenvy::dotenv().ok();

    match chain_id {
        ChainId::Sepolia | ChainId::Ethereum => {
            std::env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY must be set")
        }
        ChainId::Polygon => {
            std::env::var("POLYSCAN_API_KEY").expect("POLYSCAN_API_KEY must be set")
        }
        _ => unimplemented!(),
    }
}
