use chaindexing::{ContractState, EventContext, EventHandler};

use crate::contract::states::{Game, GamePlay};

pub struct GamePlayCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayCreatedEventHandler {
    async fn handle_event<'a>(&self, event_context: EventContext<'a>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let id = event_params.get("id").unwrap().clone().into_uint().unwrap().as_u32() as u16;
        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let coin_side =
            event_params.get("coinSide").unwrap().clone().into_uint().unwrap().as_usize() as u8;
        let play_hash = std::str::from_utf8(
            &event_params.get("playHash").unwrap().clone().into_fixed_bytes().unwrap(),
        )
        .unwrap()
        .to_string()
        .replace("\0", "");

        GamePlay {
            id,
            game_id,
            coin_side,
            play_hash,
        }
        .create(&event_context)
        .await;

        let game = Game::read_one(
            [("id".to_string(), game_id.to_string())].into(),
            &event_context,
        )
        .await
        .unwrap();

        let updates = [("play_count".to_string(), (game.play_count + 1).to_string())];
        game.update(updates.into(), &event_context).await;
    }
}
