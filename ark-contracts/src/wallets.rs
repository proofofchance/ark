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
            &ChainId::Local.get_contract_address("WALLETS"),
            &chaindexing::ChainId::Dev,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &ChainId::Sepolia.get_contract_address("WALLETS"),
                &chaindexing::ChainId::Sepolia,
                ChainId::Sepolia.get_start_block_number("WALLETS"),
            )
            .add_address(
                &ChainId::Polygon.get_contract_address("WALLETS"),
                &chaindexing::ChainId::Polygon,
                ChainId::Polygon.get_start_block_number("WALLETS"),
            )
            .add_address(
                &ChainId::Ethereum.get_contract_address("WALLETS"),
                &chaindexing::ChainId::Mainnet,
                ChainId::Ethereum.get_start_block_number("WALLETS"),
            )
    } else {
        contract
    }
}
