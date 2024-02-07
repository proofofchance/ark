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
        //     5234697,
        // )
        // .add_address(
        //     &get_contract_address(&Chain::Polygon),
        //     &chaindexing::Chain::Polygon,
        //     5234697,
        // )
    } else {
        contract
    }
}

pub fn get_contract_address(chain: &Chain) -> String {
    dotenvy::dotenv().ok();

    match chain {
        Chain::Local => std::env::var("LOCAL_COINFLIP_CONTRACT_ADDRESS")
            .expect("LOCAL_COINFLIP_CONTRACT_ADDRESS must be set"),
        Chain::LocalAlt => std::env::var("LOCAL_COINFLIP_CONTRACT_ADDRESS")
            .expect("LOCAL_COINFLIP_CONTRACT_ADDRESS must be set"),
        Chain::Ethereum => std::env::var("ETHEREUM_COINFLIP_CONTRACT_ADDRESS")
            .expect("ETHEREUM_COINFLIP_CONTRACT_ADDRESS must be set"),
        Chain::Polygon => std::env::var("POLYGON_COINFLIP_CONTRACT_ADDRESS")
            .expect("POLYGON_COINFLIP_CONTRACT_ADDRESS must be set"),
        Chain::Sepolia => std::env::var("SEPOLIA_COINFLIP_CONTRACT_ADDRESS")
            .expect("SEPOLIA_COINFLIP_CONTRACT_ADDRESS must be set"),
        _ => unimplemented!("Unsupported chain"),
    }
}
