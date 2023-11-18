use std::collections::HashMap;

use chaindexing::{ContractState, EventContext, EventHandler};

use crate::contract::states::{Game, GamePlay};

pub struct GamePlayCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayCreatedEventHandler {
    async fn handle_event<'a>(&self, event_context: EventContext<'a>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as u16;
        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let coin_side =
            event_params.get("coinSide").unwrap().clone().into_uint().unwrap().as_usize() as u8;
        let play_hash = std::str::from_utf8(
            &event_params.get("playHash").unwrap().clone().into_fixed_bytes().unwrap(),
        )
        .unwrap()
        .to_string()
        .replace("\0", "");

        let new_game_play = GamePlay {
            id,
            game_id,
            coin_side,
            play_hash,
        };

        new_game_play.create(&event_context).await;

        let game = Game::read_one(
            [("id".to_string(), game_id.to_string())].into(),
            &event_context,
        )
        .await
        .unwrap();

        let new_play_count = game.play_count + 1;
        let is_completed = new_play_count == game.max_play_count;

        let mut updates = HashMap::from([
            ("play_count".to_string(), new_play_count.to_string()),
            ("is_completed".to_string(), is_completed.to_string()),
        ]);

        let game_plays = GamePlay::read_many(
            [("game_id".to_string(), game_id.to_string())].into(),
            &event_context,
        )
        .await;

        let played_coin_sides = game_plays.iter().map(|game_play| game_play.coin_side).collect();

        if let Some(unavailable_coin_side) = game.get_unavailable_coin_side(&played_coin_sides) {
            updates.insert(
                "unavailable_coin_side".to_string(),
                (unavailable_coin_side as usize).to_string(),
            );
        }

        game.update(updates, &event_context).await;
    }
}
