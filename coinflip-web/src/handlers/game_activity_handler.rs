use std::collections::HashSet;

use crate::handlers;
use ark_web_app::AppState;
use axum::{
    extract::{Path, State},
    Json,
};

use coinflip::{GameActivity, GameStatus};
use coinflip_repo::GetGamesParams;

/// Returns all game activities in ongoing games the player is part of
/// The client can then store the last block number of the read game activity to track
/// game activities that are new or stale
pub async fn get_ongoing_game_activities(
    State(app_state): State<AppState>,
    Path(player_address): Path<String>,
) -> Result<Json<Vec<GameActivity>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let all_ongoing_games = coinflip_repo::get_games(
        &mut conn,
        &GetGamesParams {
            status: Some(GameStatus::Ongoing),
            ..Default::default()
        },
    )
    .await;

    let all_game_plays = coinflip_repo::get_game_plays_for_player(&mut conn, &player_address).await;
    let game_ids_and_chain_ids_of_games_played: HashSet<_> = all_game_plays
        .iter()
        .map(|game_play| (game_play.game_id, game_play.chain_id))
        .collect();

    let (ongoing_game_ids, ongoing_game_chain_ids): (Vec<_>, Vec<_>) = all_ongoing_games
        .iter()
        .map(|game| (game.id, game.chain_id))
        .filter(|(game_id, chain_id)| {
            game_ids_and_chain_ids_of_games_played.contains(&(*game_id, *chain_id))
        })
        .unzip();

    let mut game_activities =
        coinflip_repo::get_game_activities(&mut conn, &ongoing_game_ids, &ongoing_game_chain_ids)
            .await;
    let game_expired_activities: Vec<_> = all_ongoing_games
        .iter()
        .filter(|game| game.is_expired())
        .map(|game| GameActivity::new_expired(game.id, game.chain_id, game.expiry_timestamp))
        .collect();

    game_activities.extend(game_expired_activities);

    Ok(Json(game_activities))
}

pub async fn get_game_activities(
    State(app_state): State<AppState>,
    Path((game_id, chain_id)): Path<(u64, u64)>,
) -> Result<Json<Vec<GameActivity>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let game_id = game_id as i64;
    let chain_id = chain_id as i64;

    let mut game_activities =
        coinflip_repo::get_game_activities(&mut conn, &vec![game_id], &vec![chain_id]).await;
    if let Some(game) = coinflip_repo::get_game(&mut conn, game_id, chain_id).await {
        if game.is_expired() {
            game_activities.push(GameActivity::new_expired(
                game_id,
                game.chain_id,
                game.expiry_timestamp,
            ))
        }
    }

    Ok(Json(game_activities))
}
