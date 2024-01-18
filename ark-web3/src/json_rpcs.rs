use crate::chains::Chain;

pub fn get_url(chain: &Chain) -> String {
    match chain {
        Chain::Local => get_local_url(),
        Chain::LocalAlt => get_local_url(),
        Chain::Binance => get_binance_url(),
        Chain::Polygon => get_polygon_url(),
        Chain::Sepolia => get_sepolia_url(),
        _ => unreachable!("In valid chain id"),
    }
}

pub fn get_local_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("LOCAL_JSON_RPC_URL").expect("LOCAL_JSON_RPC_URL must be set")
}

pub fn get_binance_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("BINANCE_JSON_RPC_URL").expect("BINANCE_JSON_RPC_URL must be set")
}

pub fn get_polygon_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("POLYGON_JSON_RPC_URL").expect("POLYGON_JSON_RPC_URL must be set")
}

pub fn get_sepolia_url() -> String {
    dotenvy::dotenv().ok();

    std::env::var("SEPOLIA_JSON_RPC_URL").expect("SEPOLIA_JSON_RPC_URL must be set")
}
