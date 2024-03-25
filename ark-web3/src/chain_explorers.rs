use std::collections::HashMap;

use eyre::Result;
use serde_json::Value;

use crate::{chains::ChainId, CHAIN_AGNOSTIC_MAX_GAS_PRICE_F64, GWEI_F64};

#[derive(Clone, Debug)]
pub struct GasInfo {
    pub last_block: u64,
    pub safe_gas_price: f64,
    pub fast_gas_price: f64,
}

impl Default for GasInfo {
    fn default() -> Self {
        Self {
            last_block: 0,
            safe_gas_price: CHAIN_AGNOSTIC_MAX_GAS_PRICE_F64,
            fast_gas_price: CHAIN_AGNOSTIC_MAX_GAS_PRICE_F64,
        }
    }
}

impl GasInfo {
    pub fn get_fast_price_wei(&self) -> u64 {
        (self.fast_gas_price * GWEI_F64) as u64
    }
    pub fn get_safe_price_wei(&self) -> u64 {
        (self.safe_gas_price * GWEI_F64) as u64
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
    let response = response.json::<HashMap<String, Value>>().await?;
    let gas_info = response.get("result").unwrap().clone();
    let gas_info = gas_info.as_object().unwrap();

    Ok(GasInfo {
        last_block: gas_info.get("LastBlock").unwrap().as_str().unwrap().parse().unwrap(),
        safe_gas_price: gas_info.get("SafeGasPrice").unwrap().as_str().unwrap().parse().unwrap(),
        fast_gas_price: gas_info.get("FastGasPrice").unwrap().as_str().unwrap().parse().unwrap(),
    })
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
