use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{ContractState, EventContext, EventHandler};

use crate::contract::states::Game;

pub struct ExpiredGameRefundedHandler;

#[async_trait::async_trait]
impl EventHandler for ExpiredGameRefundedHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id =
            event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u32() as i64;
        let refunded_amount_per_player = event_params
            .get("refundedAmountPerPlayer")
            .unwrap()
            .clone()
            .into_uint()
            .unwrap()
            .to_string();

        let game = Game::read_one(
            [(("id".to_string(), game_id.to_string()))].into(),
            &event_context,
        )
        .await
        .unwrap();

        game.update(
            [
                (
                    "refunded_amount_per_player".to_string(),
                    refunded_amount_per_player,
                ),
                ("refunded_at".to_string(), event.block_timestamp.to_string()),
            ]
            .into(),
            &event_context,
        )
        .await;
    }
}
