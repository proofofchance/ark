use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{utils::address_to_string, ContractState, EventContext, EventHandler};

use crate::contract::states::GamePlay;

pub struct GamePlayProofCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayProofCreatedEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let game_play_id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as u16;
        let player_address =
            address_to_string(&event_params.get("player").unwrap().clone().into_address().unwrap())
                .to_lowercase();
        let play_proof = event_params.get("playProof").unwrap().clone().to_string();

        let game_play = GamePlay::read_one(
            [
                ("id".to_string(), game_play_id.to_string()),
                ("game_id".to_string(), game_id.to_string()),
                ("player_address".to_string(), player_address.to_string()),
            ]
            .into(),
            &event_context,
        )
        .await
        .unwrap();

        game_play
            .update(
                [("play_proof".to_string(), play_proof)].into(),
                &event_context,
            )
            .await;
    }
}
