use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::Contract;

use ark_web3::chains::ChainId;

use super::event_handlers::{CreditWalletEventHandler, DebitWalletEventHandler};
use super::states::WalletMigrations;

pub fn get() -> Contract<Arc<DBPool>> {
    let contract = Contract::new("Wallets")
        .add_event(
            "event Credit(address indexed owner, uint amount)",
            CreditWalletEventHandler,
        )
        .add_event(
            "event Debit(address indexed owner, uint amount)",
            DebitWalletEventHandler,
        )
        .add_state_migrations(WalletMigrations);

    let current_environment = ark::environments::current();

    if current_environment.is_local() {
        contract.add_address(
            &get_contract_address(&ChainId::Local),
            &chaindexing::ChainId::Dev,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &get_contract_address(&ChainId::Sepolia),
                &chaindexing::ChainId::Sepolia,
                5497825,
            )
            .add_address(
                &get_contract_address(&ChainId::Polygon),
                &chaindexing::ChainId::Polygon,
                54267834,
            )
        // .add_address(
        //     &get_contract_address(&ChainId::Ethereum),
        //     &chaindexing::ChainId::Mainnet,
        //     0,
        // )
    } else {
        contract
    }
}

pub fn get_contract_address(chain: &ChainId) -> String {
    dotenvy::dotenv().ok();

    match chain {
        ChainId::Local | ChainId::LocalAlt => std::env::var("LOCAL_WALLETS_CONTRACT_ADDRESS")
            .expect("LOCAL_WALLETS_CONTRACT_ADDRESS must be set"),
        ChainId::Ethereum => std::env::var("ETHEREUM_WALLETS_CONTRACT_ADDRESS")
            .expect("ETHEREUM_WALLETS_CONTRACT_ADDRESS must be set"),
        ChainId::Polygon => std::env::var("POLYGON_WALLETS_CONTRACT_ADDRESS")
            .expect("POLYGON_WALLETS_CONTRACT_ADDRESS must be set"),
        ChainId::Sepolia => std::env::var("SEPOLIA_WALLETS_CONTRACT_ADDRESS")
            .expect("SEPOLIA_WALLETS_CONTRACT_ADDRESS must be set"),
        _ => unimplemented!("Unsupported chain"),
    }
}
