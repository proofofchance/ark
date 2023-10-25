use ark_repo::{Order, Repo};
use ark_web_app::AppState;
use axum::{
    extract::{Path, State},
    Json,
};

use coinflip::{Game, GameField};

use crate::handlers;

pub async fn get_games(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Game>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_all_games(&mut conn, (GameField::BlockNumber, Order::Desc)).await;

    Ok(Json(games))
}

pub async fn get_creator_games(
    State(app_state): State<AppState>,
    Path(creator_address): Path<String>,
) -> Result<Json<Vec<Game>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_creator_games(&mut conn, &creator_address).await;

    Ok(Json(games))
}
