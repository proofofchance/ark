use ark_web_app::AppState;
use axum::extract::{Json, Path, State};

use coinflip::UnsavedGameActivity;
use coinflip_repo::Repo;
use serde::Deserialize;

use crate::handlers;

use super::GenericMessage;

#[derive(Debug, Deserialize)]
pub struct UpdateMyGamePlayParams {
    public_address: String,
    game_play_proof: String,
}

pub async fn update_my_game_play(
    State(app_state): State<AppState>,
    Path(game_id): Path<i64>,
    Json(UpdateMyGamePlayParams {
        public_address,
        game_play_proof,
    }): Json<UpdateMyGamePlayParams>,
) -> Result<Json<GenericMessage>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    if let Some(game_play) = Repo::get_game_play(&mut conn, game_id, &public_address).await {
        Repo::update_game_play_proof(&mut conn, &game_play, game_play_proof).await;

        let game_activity = UnsavedGameActivity::new_proof_created(game_id as u64, public_address);
        Repo::create_game_activity(&mut conn, &game_activity).await
    }

    Ok(Json(GenericMessage::new("game proof publicized")))
}
