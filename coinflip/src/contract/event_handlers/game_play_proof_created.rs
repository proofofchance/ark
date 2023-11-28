use chaindexing::{utils::address_to_string, EventContext, EventHandler};

use crate::contract::states::GamePlayProof;

use super::record_new_game_activity;

pub struct GamePlayProofCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayProofCreatedEventHandler {
    async fn handle_event<'a>(&self, event_context: EventContext<'a>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let game_play_id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as u16;
        let player_address =
            address_to_string(&event_params.get("player").unwrap().clone().into_address().unwrap())
                .to_lowercase();
        let play_proof = std::str::from_utf8(
            &event_params.get("playProof").unwrap().clone().into_fixed_bytes().unwrap(),
        )
        .unwrap()
        .to_string();

        GamePlayProof {
            game_id,
            game_play_id,
            player_address,
            play_proof,
        };

        record_new_game_activity(game_id, event.block_timestamp as u64, &event_context).await;
    }
}
