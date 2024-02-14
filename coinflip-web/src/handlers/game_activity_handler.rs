use std::collections::HashSet;

use crate::handlers;
use ark_web_common::AppState;
use axum::{
    extract::{Path, State},
    Json,
};

use coinflip::{GameActivity, GameStatus};
use coinflip_repo::GetGamesParams;

/// Returns all game activities in games the player is part of
/// The client can then store the last block number of the read game activity to track
/// game activities that are new or stale
/// TODO: Refactor and allow sending notifications via Websocket
pub async fn get_all_game_activites(
    State(app_state): State<AppState>,
    Path((game_status, player_address)): Path<(GameStatus, String)>,
) -> Result<Json<Vec<GameActivity>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let all_games = coinflip_repo::get_games(
        &mut conn,
        &GetGamesParams {
            status: Some(game_status),
            ..Default::default()
        },
    )
    .await;

    let all_game_plays = coinflip_repo::get_game_plays_for_player(&mut conn, &player_address).await;
    let game_ids_and_chain_ids_of_games_played: HashSet<_> = all_game_plays
        .iter()
        .map(|game_play| (game_play.game_id, game_play.chain_id))
        .collect();

    let (game_ids, game_chain_ids): (Vec<_>, Vec<_>) = all_games
        .iter()
        .map(|game| (game.id, game.chain_id))
        .filter(|(game_id, chain_id)| {
            game_ids_and_chain_ids_of_games_played.contains(&(*game_id, *chain_id))
        })
        .unzip();

    let game_activities =
        coinflip_repo::get_game_activities(&mut conn, &game_ids, &game_chain_ids).await;

    Ok(Json(game_activities))
}

pub async fn get_game_activities(
    State(app_state): State<AppState>,
    Path((game_id, chain_id)): Path<(u64, u64)>,
) -> Result<Json<Vec<GameActivity>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let game_id = game_id as i64;
    let chain_id = chain_id as i64;

    let game_activities =
        coinflip_repo::get_game_activities(&mut conn, &vec![game_id], &vec![chain_id]).await;

    Ok(Json(game_activities))
}
