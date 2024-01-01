use ethers::types::Chain;
use ethers::{
    core::k256::ecdsa,
    signers::{LocalWallet, Signer, Wallet},
};

pub fn get_json_rpc(chain_id: &Chain) -> String {
    match chain_id {
        Chain::Dev => get_local_json_rpc_url(),
        _ => unreachable!("In valid chain id"),
    }
}

pub fn get_local_json_rpc_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("JSON_RPC_URL").expect("JSON_RPC_URL must be set")
}

pub fn get_ark_wallet(chain_id: u64) -> Wallet<ecdsa::SigningKey> {
    let chain_id = if chain_id == 1337 { 31337 } else { chain_id };
    get_ark_private_key().parse::<LocalWallet>().unwrap().with_chain_id(chain_id)
}

fn get_ark_private_key() -> String {
    dotenvy::dotenv().ok();

    std::env::var("ARK_PRIVATE_KEY").expect("ARK_PRIVATE_KEY must be set")
}
