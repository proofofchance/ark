use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{ContractState, EventContext, EventHandler};
use coinflip::GamePlayStatus;

use crate::coinflip::states::{Game, GamePlay};

pub struct ExpiredGameRefundedHandler;

#[async_trait::async_trait]
impl EventHandler for ExpiredGameRefundedHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a, 'b>(&self, event_context: EventContext<'a, 'b, Self::SharedState>) {
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

        // get all the plays in that game
        // update their statuses to expired
        let game_plays = GamePlay::read_many(
            [(("game_id".to_string(), game_id.to_string()))].into(),
            &event_context,
        )
        .await;

        for game_play in game_plays {
            game_play
                .update(
                    [("status".to_string(), GamePlayStatus::Expired.into())].into(),
                    &event_context,
                )
                .await;
        }
    }
}
