mod event_handlers;
mod states;

use chaindexing::{Chain, Contract};

use event_handlers::{
    GameCreatedEventHandler, GamePlayCreatedEventHandler, GamePlayProofCreatedEventHandler,
};

use states::{GameActivityMigrations, GameMigrations, GamePlayMigrations};

use dotenvy::dotenv;

pub struct CoinflipContract;

impl CoinflipContract {
    pub fn get() -> Contract {
        Contract::new("Coinflip")
        .add_event(
            "event GameCreated(uint256 gameID, uint16 maxPlayCount, uint256 expiryTimestamp, address creator, uint256 wager)",
            GameCreatedEventHandler,
        )
        .add_event(
            "event GamePlayCreated(uint16 gamePlayID, uint256 gameID, uint8 coinSide, address player, bytes32 playHash)",
            GamePlayCreatedEventHandler,
        )
        .add_event(
            "event GamePlayProofCreated(uint16 gamePlayID, uint256 gameID, address player, string playProof)",
            GamePlayProofCreatedEventHandler,
        )
        .add_state_migrations(GameMigrations)
        .add_state_migrations(GamePlayMigrations)
        .add_state_migrations(GameActivityMigrations)
        .add_address(&Self::address(), &Chain::Dev, 0)
    }

    fn address() -> String {
        dotenv().ok();

        std::env::var("COINFLIP_ADDRESS").expect("COINFLIP_ADDRESS must be set")
    }
}
