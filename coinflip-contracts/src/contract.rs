mod event_handlers;
mod states;

use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{Chain, Contract};

use event_handlers::{
    GameCompletedEventHandler, GameCreatedEventHandler, GamePlayCreatedEventHandler,
};

use states::{GameMigrations, GamePlayMigrations};

use dotenvy::dotenv;

use self::event_handlers::GamePlayChanceRevealedEventHandler;

pub fn get() -> Contract<Arc<DBPool>> {
    Contract::new("Coinflip")
        .add_event(
            "event GameCreated(uint256 gameID, uint16 numberOfPlayers, uint256 expiryTimestamp, address creator, uint256 wager)",
            GameCreatedEventHandler,
        )
        .add_event(
            "event GamePlayCreated(uint256 gameID, uint16 gamePlayID, uint8 coinSide, address player, bytes32 proofOfChance)",
            GamePlayCreatedEventHandler,
        )
        .add_event(
            "event GameCompleted(uint256 gameID, uint8 coinSide)",
            GameCompletedEventHandler,
        )
        .add_event("event GamePlayChanceRevealed(uint gameID, uint16 gamePlayID, bytes chanceAndSalt)", GamePlayChanceRevealedEventHandler)
        .add_state_migrations(GameMigrations)
        .add_state_migrations(GamePlayMigrations)
        .add_address(&get_coinflip_contract_address(), &Chain::Dev, 0)
}

pub fn get_coinflip_contract_address() -> String {
    dotenv().ok();

    std::env::var("COINFLIP_ADDRESS").expect("COINFLIP_ADDRESS must be set")
}
