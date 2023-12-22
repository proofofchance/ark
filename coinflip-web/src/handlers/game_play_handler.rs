use ark_web_app::AppState;
use axum::extract::{Json, Path, State};

use coinflip_repo::Repo;
use serde::Deserialize;

use crate::handlers;

use super::GenericMessage;

#[derive(Debug, Deserialize)]
pub struct UpdateGamePlayProofParams {
    public_address: String,
    game_play_proof: String,
}

pub async fn update_game_play_proof(
    State(app_state): State<AppState>,
    Path(game_id): Path<i64>,
    Json(UpdateGamePlayProofParams {
        public_address,
        game_play_proof,
    }): Json<UpdateGamePlayProofParams>,
) -> Result<Json<GenericMessage>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    if let Some(game_play) = Repo::get_game_play(&mut conn, game_id, &public_address).await {
        Repo::update_game_play_proof(&mut conn, &game_play, game_play_proof).await;
    }

    Ok(Json(GenericMessage::new("game proof publicized")))
}
