use ethers::{
    core::k256::ecdsa,
    signers::{LocalWallet, Signer, Wallet},
};

use crate::chains::ChainId;

pub fn get(chain: &ChainId) -> Wallet<ecdsa::SigningKey> {
    let chain_id = *chain as usize as u64;
    let chain_id = if chain_id == 1337 { 31337 } else { chain_id };
    get_private_key(chain).parse::<LocalWallet>().unwrap().with_chain_id(chain_id)
}

fn get_private_key(chain: &ChainId) -> String {
    dotenvy::dotenv().ok();

    match chain {
        ChainId::Local | ChainId::LocalAlt => {
            std::env::var("LOCAL_PRIVATE_KEY").expect("LOCAL_PRIVATE_KEY must be set")
        }
        ChainId::Ethereum => {
            std::env::var("ETHEREUM_PRIVATE_KEY").expect("ETHEREUM_PRIVATE_KEY must be set")
        }
        ChainId::Polygon => {
            std::env::var("POLYGON_PRIVATE_KEY").expect("POLYGON_PRIVATE_KEY must be set")
        }
        ChainId::Sepolia => {
            std::env::var("SEPOLIA_PRIVATE_KEY").expect("SEPOLIA_PRIVATE_KEY must be set")
        }
        _ => unimplemented!("Invalid chain id"),
    }
}
