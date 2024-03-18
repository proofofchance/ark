use crate::chains::ChainId;

pub fn get_url(chain: &ChainId) -> String {
    match chain {
        ChainId::Local | ChainId::LocalAlt => get_local_url(),
        ChainId::Ethereum => get_ethereum_url(),
        ChainId::Polygon => get_polygon_url(),
        ChainId::Sepolia => get_sepolia_url(),
        _ => unreachable!("In valid chain id"),
    }
}

pub fn get_local_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("LOCAL_JSON_RPC_URL").expect("LOCAL_JSON_RPC_URL must be set")
}

pub fn get_ethereum_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("ETHEREUM_JSON_RPC_URL").expect("ETHEREUM_JSON_RPC_URL must be set")
}

pub fn get_polygon_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("POLYGON_JSON_RPC_URL").expect("POLYGON_JSON_RPC_URL must be set")
}

pub fn get_sepolia_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("SEPOLIA_JSON_RPC_URL").expect("SEPOLIA_JSON_RPC_URL must be set")
}
