use ethers::{
    core::k256::ecdsa,
    signers::{LocalWallet, Signer, Wallet},
};

use crate::chains::Chain;

pub fn get(chain: &Chain) -> Wallet<ecdsa::SigningKey> {
    let chain_id = *chain as usize as u64;
    let chain_id = if chain_id == 1337 { 31337 } else { chain_id };
    get_private_key(chain).parse::<LocalWallet>().unwrap().with_chain_id(chain_id)
}

fn get_private_key(chain: &Chain) -> String {
    dotenvy::dotenv().ok();

    match chain {
        Chain::Local => {
            std::env::var("ARK_LOCAL_PRIVATE_KEY").expect("ARK_LOCAL_PRIVATE_KEY must be set")
        }
        Chain::LocalAlt => {
            std::env::var("ARK_LOCAL_PRIVATE_KEY").expect("ARK_LOCAL_PRIVATE_KEY must be set")
        }
        Chain::Binance => {
            std::env::var("ARK_BINANCE_PRIVATE_KEY").expect("ARK_BINANCE_PRIVATE_KEY must be set")
        }
        Chain::Polygon => {
            std::env::var("ARK_POLYGON_PRIVATE_KEY").expect("ARK_POLYGON_PRIVATE_KEY must be set")
        }
        Chain::Sepolia => {
            std::env::var("ARK_SEPOLIA_PRIVATE_KEY").expect("ARK_SEPOLIA_PRIVATE_KEY must be set")
        }
        _ => unimplemented!("Invalid chain id"),
    }
}
