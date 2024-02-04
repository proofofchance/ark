mod event_handlers;
mod states;

use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::Contract;

use ark_web3::chains::Chain;

use event_handlers::{
    ExpiredGameRefundedHandler, GameCompletedEventHandler, GameCreatedEventHandler,
    GameExpiryAdjustedHandler, GamePlayChanceRevealedEventHandler, GamePlayCreatedEventHandler,
};

use serde::Deserialize;
use states::{GameMigrations, GamePlayMigrations};

pub fn get() -> Contract<Arc<DBPool>> {
    let mut contract = Contract::new("Coinflip")
        .add_event(
            "event GameCreated(uint256 indexed gameID, address indexed creator, uint16 numberOfPlayers, uint256 expiryTimestamp, uint256 wager)",
            GameCreatedEventHandler,
        )
        .add_event(
            "event GamePlayCreated(uint256 indexed gameID, uint16 indexed gamePlayID, address indexed player, uint8 coinSide, bytes32 proofOfChance)",
            GamePlayCreatedEventHandler,
        )
        .add_event(
            "event GameCompleted(uint256 indexed gameID, uint8 coinSide, uint amountForEachWinner)",
            GameCompletedEventHandler,
        )
        .add_event("event GamePlayChanceRevealed(uint indexed gameID, uint16 indexed gamePlayID, bytes chanceAndSalt)", GamePlayChanceRevealedEventHandler)
        .add_event("event ExpiredGameRefunded(uint indexed gameID, uint refundedAmountPerPlayer)", ExpiredGameRefundedHandler)
        .add_event("event GameExpiryAdjusted(uint indexed gameID, uint expiryTimestamp)", GameExpiryAdjustedHandler)
        .add_state_migrations(GameMigrations)
        .add_state_migrations(GamePlayMigrations);

    let current_environment = ark::environments::current();

    if current_environment.is_local() {
        contract.add_address(
            &CoinflipContractAddress::get(&Chain::Local),
            &chaindexing::Chain::Dev,
            0,
        )
    } else if current_environment.is_staging() {
        contract.add_address(
            &CoinflipContractAddress::get(&Chain::Sepolia),
            &chaindexing::Chain::Sepolia,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &CoinflipContractAddress::get(&Chain::Binance),
                &chaindexing::Chain::BinanceSmartChain,
                0,
            )
            .add_address(
                &CoinflipContractAddress::get(&Chain::Polygon),
                &chaindexing::Chain::Polygon,
                0,
            )
    } else {
        contract
    }
}

#[derive(Deserialize)]
pub struct CoinflipContractAddress {
    address: String,
}

impl CoinflipContractAddress {
    pub fn get(chain: &Chain) -> String {
        CoinflipContractAddress::new(chain).address
    }
    fn new(chain: &Chain) -> CoinflipContractAddress {
        let deployed_abi_string = Self::get_deployed_abi_string(chain);
        serde_json::from_str(&deployed_abi_string).unwrap()
    }

    fn get_deployed_abi_string(chain: &Chain) -> String {
        match chain {
            Chain::Local => include_str!(
                "../../../orisirisi/libs/coinflip-contracts/deployments/localhost/Coinflip.json"
            ),
            Chain::LocalAlt => include_str!(
                "../../../orisirisi/libs/coinflip-contracts/deployments/localhost/Coinflip.json"
            ),

            // TODO: Add back once deployed on these networks
            // Chain::Binance =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/binance/Coinflip.json"
            //     )

            // Chain::Polygon =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/polygon/Coinflip.json"
            //     )

            // Chain::SepoliaTestNet =>
            //     include_str!(
            //         "../../../orisirisi/libs/coinflip-contracts/deployments/sepolia/Coinflip.json"
            //     )
            _ => panic!("Unsupported Chain"),
        }
        .to_string()
    }
}
