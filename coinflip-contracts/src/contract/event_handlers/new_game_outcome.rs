use std::sync::Arc;

use ark_db::DBPool;

use chaindexing::{ContractState, EventContext, EventHandler};

use crate::contract::states::{Game, GamePlay};
use coinflip::GamePlayStatus;

pub struct NewGameOutcomeEventHandler;

#[async_trait::async_trait]
impl EventHandler for NewGameOutcomeEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let outcome_coin_side =
            event_params.get("coinSide").unwrap().clone().into_uint().unwrap().as_usize() as u8;

        let game = Game::read_one(
            [("id".to_string(), game_id.to_string())].into(),
            &event_context,
        )
        .await
        .unwrap();
        game.update(
            [
                ("outcome".to_string(), outcome_coin_side.to_string()),
                (
                    "completed_at".to_string(),
                    event_context.event.block_number.to_string(),
                ),
            ]
            .into(),
            &event_context,
        )
        .await;

        // get all the plays in that game
        // update their statuses to win or lose
        let game_plays = GamePlay::read_many(
            [(("game_id".to_string(), game_id.to_string()))].into(),
            &event_context,
        )
        .await;

        for game_play in game_plays {
            let game_play_status = if game_play.coin_side == outcome_coin_side {
                GamePlayStatus::Won
            } else {
                GamePlayStatus::Lost
            };

            game_play
                .update(
                    [("status".to_string(), game_play_status.into())].into(),
                    &event_context,
                )
                .await;
        }
    }
}
