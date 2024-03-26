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
            &ChainId::Local.get_contract_address("COINFLIP"),
            &chaindexing::ChainId::Dev,
            0,
        )
    } else if current_environment.is_production() {
        contract
            .add_address(
                &ChainId::Sepolia.get_contract_address("COINFLIP"),
                &chaindexing::ChainId::Sepolia,
                ChainId::Sepolia.get_start_block_number("COINFLIP"),
            )
            .add_address(
                &ChainId::Polygon.get_contract_address("COINFLIP"),
                &chaindexing::ChainId::Polygon,
                ChainId::Polygon.get_start_block_number("COINFLIP"),
            )
            .add_address(
                &ChainId::Ethereum.get_contract_address("COINFLIP"),
                &chaindexing::ChainId::Mainnet,
                ChainId::Ethereum.get_start_block_number("COINFLIP"),
            )
    } else {
        contract
    }
}
