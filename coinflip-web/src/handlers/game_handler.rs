use ark_repo::{GetGamesParams, Repo};
use ark_web_app::AppState;
use axum::{
    extract::{Query, State},
    Json,
};

use coinflip::Game;

use crate::handlers;

pub async fn get_games(
    State(app_state): State<AppState>,
    query_params: Query<GetGamesParams>,
) -> Result<Json<Vec<Game>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_all_games(&mut conn, &query_params).await;

    Ok(Json(games))
}
