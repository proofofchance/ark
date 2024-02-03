use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::Contract;

use ark_web3::chains::Chain;

use super::event_handlers::{CreditWalletEventHandler, DebitWalletEventHandler};
use super::states::WalletMigrations;

use serde::Deserialize;

pub fn get() -> Contract<Arc<DBPool>> {
    let mut contract = Contract::new("Wallets")
        .add_event(
            "event Credit(address indexed owner, uint amount)",
            CreditWalletEventHandler,
        )
        .add_event(
            "event CreditFromGame(address indexed app, uint indexed gameID, address indexed owner, uint amount)",
            CreditWalletEventHandler,
        )
        .add_event(
            "event Debit(address indexed owner, uint amount)",
            DebitWalletEventHandler,
        )
        .add_event(
            "event DebitForGame(address indexed app, uint indexed gameID, address indexed owner, uint amount)",
            DebitWalletEventHandler,
        )
        .add_state_migrations(WalletMigrations);

    let current_environment = ark::environments::current();

    if current_environment.is_local() {
        contract.add_address(
            &WalletsContractAddress::get(&Chain::Local),
            &chaindexing::Chain::Dev,
            0,
        )
    } else if current_environment.is_staging() {
        contract.add_address(
            &WalletsContractAddress::get(&Chain::Sepolia),
            &chaindexing::Chain::Sepolia,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &WalletsContractAddress::get(&Chain::Binance),
                &chaindexing::Chain::BinanceSmartChain,
                0,
            )
            .add_address(
                &WalletsContractAddress::get(&Chain::Polygon),
                &chaindexing::Chain::Polygon,
                0,
            )
    } else {
        contract
    }
}

#[derive(Deserialize)]
pub struct WalletsContractAddress {
    address: String,
}

impl WalletsContractAddress {
    pub fn get(chain: &Chain) -> String {
        WalletsContractAddress::new(chain).address
    }
    fn new(chain: &Chain) -> WalletsContractAddress {
        let deployed_abi_string = Self::get_deployed_abi_string(chain);
        serde_json::from_str(&deployed_abi_string).unwrap()
    }

    fn get_deployed_abi_string(chain: &Chain) -> String {
        match chain {
            Chain::Local => include_str!(
                "../../../orisirisi/libs/coinflip-contracts/deployments/localhost/Wallets.json"
            ),
            Chain::LocalAlt => include_str!(
                "../../../orisirisi/libs/coinflip-contracts/deployments/localhost/Wallets.json"
            ),

            // TODO: Add back once deployed on these networks
            // Chain::Binance =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/binance/Wallets.json"
            //     )

            // Chain::Polygon =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/polygon/Wallets.json"
            //     )

            // Chain::SepoliaTestNet =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/sepolia/Wallets.json"
            //     )
            _ => panic!("Unsupported Chain"),
        }
        .to_string()
    }
}
