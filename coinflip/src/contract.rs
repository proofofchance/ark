use chaindexing::utils::address_to_string;

use chaindexing::{
    Chain, Contract, ContractState, ContractStateMigrations, EventContext, EventHandler,
};
use serde::{Deserialize, Serialize};

use dotenvy::dotenv;

pub struct CoinflipContract;

impl CoinflipContract {
    pub fn get() -> Contract {
        Contract::new("Coinflip")
        .add_event(
            "event GameCreated(uint256 gameID, uint16 maxPlayCount, uint256 expiryTimestamp, address creator, uint256 wager)",
            GameCreatedEventHandler,
        )
        .add_state_migrations(GamesMigrations)
        .add_address(&Self::address(), &Chain::Dev, 0)
    }

    fn address() -> String {
        dotenv().ok();

        std::env::var("COINFLIP_ADDRESS").expect("COINFLIP_ADDRESS must be set")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    id: u64,
    max_play_count: u32,
    expiry_timestamp: u64,
    creator_address: String,
    wager: u64,
}

impl ContractState for Game {
    fn table_name() -> &'static str {
        "coinflip_games"
    }
}

struct GamesMigrations;

impl ContractStateMigrations for GamesMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_games (
                id BIGINT NOT NULL,
                max_play_count INTEGER NOT NULL,
                expiry_timestamp BIGINT NOT NULL,
                creator_address VARCHAR NOT NULL,
                wager BIGINT NOT NULL
            )",
        ]
    }
}

struct GameCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GameCreatedEventHandler {
    async fn handle_event<'a>(&self, event_context: EventContext<'a>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let max_play_count =
            event_params.get("maxPlayCount").unwrap().clone().into_uint().unwrap().as_u32();
        let expiry_timestamp = event_params
            .get("expiryTimestamp")
            .unwrap()
            .clone()
            .into_uint()
            .unwrap()
            .as_u64();
        let creator_address = address_to_string(
            &event_params.get("creator").unwrap().clone().into_address().unwrap(),
        )
        .to_lowercase();
        let wager = event_params.get("wager").unwrap().clone().into_uint().unwrap().as_u64();

        Game {
            id,
            max_play_count,
            expiry_timestamp,
            creator_address,
            wager,
        }
        .create(&event_context)
        .await;
    }
}
