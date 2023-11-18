use chaindexing::{utils::address_to_string, ContractState, EventContext, EventHandler};

use crate::contract::states::Game;

pub struct GameCreatedEventHandler;

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
        let wager = event_params.get("wager").unwrap().clone().into_uint().unwrap().to_string();

        Game {
            id,
            max_play_count,
            expiry_timestamp,
            creator_address,
            wager,
            play_count: 0,
            head_play_count: 0,
            tail_play_count: 0,
            is_completed: false,
            unavailable_coin_side: None,
        }
        .create(&event_context)
        .await;
    }
}
