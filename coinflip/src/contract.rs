use chaindexing::{
    Chain, Contract, ContractState, ContractStateMigrations, EventContext, EventHandler,
};
use serde::{Deserialize, Serialize};

pub fn get_coinflip_contract() -> Contract {
    Contract::new("Coinflip")
        .add_event(
            "event GameCreated(uint256 gameID, uint16 maxPlayCount, uint256 expiryTimestamp)",
            GameCreatedEventHandler,
        )
        .add_state_migrations(GamesMigrations)
        .add_address("0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0", &Chain::Dev, 0)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Game {
    id: u64,
    max_play_count: u32,
    expiry_timestamp: u64,
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
                expiry_timestamp BIGINT NOT NULL
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

        Game {
            id,
            max_play_count,
            expiry_timestamp,
        }
        .create(&event_context)
        .await;
    }
}
