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
            std::env::var("LOCAL_ARK_PRIVATE_KEY").expect("LOCAL_ARK_PRIVATE_KEY must be set")
        }
        Chain::LocalAlt => {
            std::env::var("LOCAL_ARK_PRIVATE_KEY").expect("LOCAL_ARK_PRIVATE_KEY must be set")
        }
        Chain::Binance => {
            std::env::var("BINANCE_ARK_PRIVATE_KEY").expect("BINANCE_ARK_PRIVATE_KEY must be set")
        }
        Chain::Polygon => {
            std::env::var("POLYGON_ARK_PRIVATE_KEY").expect("POLYGON_ARK_PRIVATE_KEY must be set")
        }
        Chain::Sepolia => {
            std::env::var("SEPOLIA_ARK_PRIVATE_KEY").expect("SEPOLIA_ARK_PRIVATE_KEY must be set")
        }
        _ => unimplemented!("Invalid chain id"),
    }
}
