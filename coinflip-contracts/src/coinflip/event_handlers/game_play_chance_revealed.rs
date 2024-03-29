use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{EventContext, EventHandler};
use coinflip::{GameActivityKind, UnsavedGameActivity};
use coinflip_repo::GetGameActivityParams;

use crate::coinflip::states::GamePlay;

use chaindexing::ContractState;
pub struct GamePlayChanceRevealedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayChanceRevealedEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a, 'b>(&self, event_context: EventContext<'a, 'b, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let game_id =
            event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u32() as i64;
        let game_play_id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as i32;
        let chance_and_salt = event_params.get("chanceAndSalt").unwrap().to_string();

        let game_play = GamePlay::read_one(
            [
                ("id".to_string(), game_play_id.to_string()),
                ("game_id".to_string(), game_id.to_string()),
            ]
            .into(),
            &event_context,
        )
        .await
        .unwrap();

        game_play
            .update(
                [(
                    "chance_and_salt".to_string(),
                    format!("0x{chance_and_salt}"),
                )]
                .into(),
                &event_context,
            )
            .await;

        let pool = event_context.get_shared_state().await;
        let mut conn = pool.get_owned().await.unwrap();

        let game_activity = UnsavedGameActivity::new_chance_revealed(
            game_id as u64,
            event.chain_id,
            &game_play.player_address,
        )
        .with_transaction_hash(&event.transaction_hash);

        if let Some(game_activity) = coinflip_repo::get_game_activity(
            &mut conn,
            &GetGameActivityParams {
                game_id: game_id as i64,
                chain_id: event.chain_id as i64,
                kind: GameActivityKind::GamePlayChanceRevealed.into(),
                trigger_public_address: game_play.player_address.to_owned(),
            },
        )
        .await
        {
            coinflip_repo::update_game_activity_transaction_hash(
                &mut conn,
                &game_activity,
                &event.transaction_hash,
            )
            .await;
        } else {
            coinflip_repo::create_game_activity(&mut conn, &game_activity).await;
        }
    }
}
