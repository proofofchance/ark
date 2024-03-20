mod event_handlers;
mod states;

use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::Contract;

use ark_web3::chains::ChainId;

use event_handlers::{
    ExpiredGameRefundedHandler, GameCompletedEventHandler, GameCreatedEventHandler,
    GameExpiryAdjustedHandler, GamePlayChanceRevealedEventHandler, GamePlayCreatedEventHandler,
};

use states::{GameMigrations, GamePlayMigrations};

pub fn get() -> Contract<Arc<DBPool>> {
    let contract = Contract::new("Coinflip")
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
            &get_contract_address(&ChainId::Local),
            &chaindexing::ChainId::Dev,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &get_contract_address(&ChainId::Sepolia),
                &chaindexing::ChainId::Sepolia,
                5527334,
            )
            .add_address(
                &get_contract_address(&ChainId::Polygon),
                &chaindexing::ChainId::Polygon,
                54267834,
            )
        // .add_address(
        //     &get_contract_address(&ChainId::Ethereum),
        //     &chaindexing::ChainId::Mainnet,
        //     5300263,
        // )
    } else {
        contract
    }
}

pub fn get_contract_address(chain: &ChainId) -> String {
    dotenvy::dotenv().ok();

    match chain {
        ChainId::Local | ChainId::LocalAlt => std::env::var("LOCAL_COINFLIP_CONTRACT_ADDRESS")
            .expect("LOCAL_COINFLIP_CONTRACT_ADDRESS must be set"),
        ChainId::Ethereum => std::env::var("ETHEREUM_COINFLIP_CONTRACT_ADDRESS")
            .expect("ETHEREUM_COINFLIP_CONTRACT_ADDRESS must be set"),
        ChainId::Polygon => std::env::var("POLYGON_COINFLIP_CONTRACT_ADDRESS")
            .expect("POLYGON_COINFLIP_CONTRACT_ADDRESS must be set"),
        ChainId::Sepolia => std::env::var("SEPOLIA_COINFLIP_CONTRACT_ADDRESS")
            .expect("SEPOLIA_COINFLIP_CONTRACT_ADDRESS must be set"),
        _ => unimplemented!("Unsupported chain"),
    }
}
