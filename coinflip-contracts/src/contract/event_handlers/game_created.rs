use std::sync::Arc;

use ark_db::DBPool;

use chaindexing::{utils::address_to_string, ContractState, EventContext, EventHandler};
use coinflip_repo::Repo;

use crate::contract::states::Game;
use coinflip::UnsavedGameActivity;

pub struct GameCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GameCreatedEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let pool = event_context.get_shared_state().await;

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
            creator_address: creator_address.clone(),
            wager,
            play_count: 0,
            head_play_count: 0,
            tail_play_count: 0,
            is_completed: false,
            unavailable_coin_side: None,
            winner_address: None,
        }
        .create(&event_context)
        .await;

        let mut conn = pool.get_owned().await.unwrap();
        let game_activity = UnsavedGameActivity::new_game_created(
            id,
            creator_address.clone(),
            event.block_timestamp,
            event.transaction_hash.clone(),
        );
        Repo::create_game_activity(&mut conn, &game_activity).await;
    }
}
