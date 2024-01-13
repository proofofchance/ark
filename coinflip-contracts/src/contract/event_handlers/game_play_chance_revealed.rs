use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{EventContext, EventHandler};

pub struct GamePlayChanceRevealedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayChanceRevealedEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id =
            event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u32() as i64;
        let game_play_id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as i32;
        let chance_and_salt = event_params.get("chanceAndSalt").unwrap().to_string();

        let pool = event_context.get_shared_state().await;
        let mut conn = pool.get_owned().await.unwrap();

        let game_play =
            coinflip_repo::get_game_play_by_id(&mut conn, game_id, event.chain_id, game_play_id)
                .await
                .unwrap();

        if game_play.chance_and_salt.is_none() {
            coinflip_repo::update_game_play_chance_and_salt(&mut conn, &game_play, chance_and_salt)
                .await;
        }
    }
}
