use std::{collections::HashMap, sync::Arc};

use ark_db::{DBConn, DBPool};
use chaindexing::{utils::address_to_string, ContractState, Event, EventContext, EventHandler};

use crate::contract::states::{Game, GamePlay};
use coinflip::{CoinSide, UnsavedGameActivity};

pub struct GamePlayCreatedEventHandler;

#[async_trait::async_trait]
impl EventHandler for GamePlayCreatedEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let id =
            event_params.get("gamePlayID").unwrap().clone().into_uint().unwrap().as_u32() as u16;
        let game_id = event_params.get("gameID").unwrap().clone().into_uint().unwrap().as_u64();
        let coin_side =
            event_params.get("coinSide").unwrap().clone().into_uint().unwrap().as_usize() as u8;
        let player_address =
            address_to_string(&event_params.get("player").unwrap().clone().into_address().unwrap())
                .to_lowercase();
        let proof_of_chance = &event_params.get("proofOfChance").unwrap().clone().to_string();

        let new_game_play = GamePlay {
            id,
            game_id,
            coin_side,
            player_address: player_address.clone(),
            proof_of_chance: proof_of_chance.clone(),
            chance_and_salt: None,
        };

        new_game_play.create(&event_context).await;

        update_game(&new_game_play, &event_context).await;

        let pool = event_context.get_shared_state().await;
        let mut conn = pool.get_owned().await.unwrap();

        create_game_activity(&mut conn, &new_game_play, event).await;
    }
}

async fn update_game<'a, S: Send + Sync + Clone>(
    new_game_play: &GamePlay,
    event_context: &EventContext<'a, S>,
) {
    let game = Game::read_one(
        [("id".to_string(), new_game_play.game_id.to_string())].into(),
        &event_context,
    )
    .await
    .unwrap();

    let new_play_count = game.play_count + 1;
    let (new_head_play_count, new_tail_play_count) =
        get_new_head_and_tail_play_counts(&new_game_play, &game);

    let mut updates = HashMap::from([
        ("play_count".to_string(), new_play_count.to_string()),
        (
            "head_play_count".to_string(),
            new_head_play_count.to_string(),
        ),
        (
            "tail_play_count".to_string(),
            new_tail_play_count.to_string(),
        ),
    ]);
    if let Some(unavailable_coin_side) = get_unavailable_coin_side(&game, &event_context).await {
        updates.insert(
            "unavailable_coin_side".to_string(),
            (unavailable_coin_side as usize).to_string(),
        );
    }

    game.update(updates, &event_context).await;
}

async fn create_game_activity<'a>(conn: &mut DBConn<'a>, new_game_play: &GamePlay, event: &Event) {
    let game_activity = UnsavedGameActivity::new_game_play_created(
        new_game_play.game_id,
        event.chain_id,
        new_game_play.player_address.clone(),
        event.block_timestamp,
        event.transaction_hash.clone(),
        new_game_play.coin_side,
        new_game_play.proof_of_chance.clone(),
    );

    coinflip_repo::create_game_activity(conn, &game_activity).await;
}

fn get_new_head_and_tail_play_counts(new_game_play: &GamePlay, game: &Game) -> (u32, u32) {
    let new_head_play_count = if CoinSide::is_head_u8(new_game_play.coin_side) {
        game.head_play_count + 1
    } else {
        game.head_play_count
    };

    let new_tail_play_count = if CoinSide::is_head_u8(new_game_play.coin_side) {
        game.tail_play_count + 1
    } else {
        game.tail_play_count
    };

    (new_head_play_count, new_tail_play_count)
}

async fn get_unavailable_coin_side<'a, S: Send + Sync + Clone>(
    game: &Game,
    event_context: &EventContext<'a, S>,
) -> Option<u8> {
    let game_plays = GamePlay::read_many(
        [("game_id".to_string(), game.id.to_string())].into(),
        &event_context,
    )
    .await;

    let played_coin_sides = game_plays.iter().map(|game_play| game_play.coin_side).collect();

    game.get_unavailable_coin_side(&played_coin_sides)
}
