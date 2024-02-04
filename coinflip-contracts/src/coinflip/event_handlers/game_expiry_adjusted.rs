use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{ContractState, EventContext, EventHandler};

use crate::coinflip::states::Game;

pub struct GameExpiryAdjustedHandler;

#[async_trait::async_trait]
impl EventHandler for GameExpiryAdjustedHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id =
            event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64() as i64;
        let expiry_timestamp = event_params
            .get("expiryTimestamp")
            .unwrap()
            .clone()
            .into_uint()
            .unwrap()
            .as_u64();

        let game = Game::read_one(
            [("id".to_string(), game_id.to_string())].into(),
            &event_context,
        )
        .await
        .unwrap();

        game.update(
            [("expiry_timestamp".to_string(), expiry_timestamp.to_string())].into(),
            &event_context,
        )
        .await;
    }
}
