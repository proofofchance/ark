mod event_handlers;
mod states;

use chaindexing::{Chain, Contract};

use event_handlers::{GameCreatedEventHandler, GamePlayCreatedEventHandler};

use states::{GamePlaysMigrations, GamesMigrations};

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
            "event GamePlayCreated(uint16 gamePlayID, uint256 gameID, uint8 coinSide, bytes32 playHash)",
            GamePlayCreatedEventHandler,
        )
        .add_state_migrations(GamesMigrations)
        .add_state_migrations(GamePlaysMigrations)
        .add_address(&Self::address(), &Chain::Dev, 0)
    }

    fn address() -> String {
        dotenv().ok();

        std::env::var("COINFLIP_ADDRESS").expect("COINFLIP_ADDRESS must be set")
    }
}
