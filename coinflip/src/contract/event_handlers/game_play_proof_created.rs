use chaindexing::{utils::address_to_string, ContractState, EventContext, EventHandler};

use crate::{
    contract::states::{GameActivity, GamePlay},
    GameActivityKind,
};

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

        GameActivity {
            game_id: game_id,
            block_timestamp: event.block_number as u64,
            trigger_public_address: player_address.clone().to_lowercase(),
            kind: GameActivityKind::GamePlayProofCreated,
            data: None,
            transaction_hash: event.transaction_hash.clone(),
        }
        .create(&event_context)
        .await;
    }
}
