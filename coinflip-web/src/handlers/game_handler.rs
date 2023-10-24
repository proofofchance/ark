use ark_repo::Repo;
use ark_web_app::AppState;
use axum::{extract::State, Json};

use coinflip::Game;

use crate::handlers;

pub async fn get_games(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Game>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_all_games(&mut conn).await;

    Ok(Json(games))
}
