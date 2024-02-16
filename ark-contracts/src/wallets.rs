use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::Contract;

use ark_web3::chains::Chain;

use super::event_handlers::{CreditWalletEventHandler, DebitWalletEventHandler};
use super::states::WalletMigrations;

pub fn get() -> Contract<Arc<DBPool>> {
    let mut contract = Contract::new("Wallets")
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
            &get_contract_address(&Chain::Local),
            &chaindexing::Chain::Dev,
            0,
        )
    } else if current_environment.is_production() {
        contract.add_address(
            &get_contract_address(&Chain::Sepolia),
            &chaindexing::Chain::Sepolia,
            5234697,
        )
        // .add_address(
        //     &get_contract_address(&Chain::Ethereum),
        //     &chaindexing::Chain::Mainnet,
        //     0,
        // )
        // .add_address(
        //     &get_contract_address(&Chain::Polygon),
        //     &chaindexing::Chain::Polygon,
        //     0,
        // )
    } else {
        contract
    }
}

pub fn get_contract_address(chain: &Chain) -> String {
    dotenvy::dotenv().ok();

    match chain {
        Chain::Local | Chain::LocalAlt => std::env::var("LOCAL_WALLETS_CONTRACT_ADDRESS")
            .expect("LOCAL_WALLETS_CONTRACT_ADDRESS must be set"),
        Chain::Ethereum => std::env::var("ETHEREUM_WALLETS_CONTRACT_ADDRESS")
            .expect("ETHEREUM_WALLETS_CONTRACT_ADDRESS must be set"),
        Chain::Polygon => std::env::var("POLYGON_WALLETS_CONTRACT_ADDRESS")
            .expect("POLYGON_WALLETS_CONTRACT_ADDRESS must be set"),
        Chain::Sepolia => std::env::var("SEPOLIA_WALLETS_CONTRACT_ADDRESS")
            .expect("SEPOLIA_WALLETS_CONTRACT_ADDRESS must be set"),
        _ => unimplemented!("Unsupported chain"),
    }
}
