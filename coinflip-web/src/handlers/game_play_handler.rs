use ark_web_app::AppState;
use axum::extract::{Json, Path, State};

use coinflip::UnsavedGameActivity;
use http::StatusCode;
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
    Path((game_id, chain_id)): Path<(u64, u64)>,
    Json(UpdateMyGamePlayParams {
        public_address,
        game_play_proof,
    }): Json<UpdateMyGamePlayParams>,
) -> Result<Json<GenericMessage>, handlers::Error> {
    let game_id = game_id as i64;
    let chain_id = chain_id as i64;

    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let maybe_game = coinflip_repo::get_game(&mut conn, game_id, chain_id).await;
    let maybe_game_play =
        coinflip_repo::get_game_play(&mut conn, game_id, chain_id, &public_address).await;

    if maybe_game.is_some()
        && maybe_game.as_ref().unwrap().is_awaiting_proofs_upload()
        && maybe_game_play.is_some()
    {
        let game_play = maybe_game_play.unwrap();

        if game_play.is_play_proof(&game_play_proof) {
            coinflip_repo::update_game_play_proof(&mut conn, &game_play, game_play_proof).await;

            let game_activity =
                UnsavedGameActivity::new_proof_created(game_id as u64, chain_id, public_address);
            coinflip_repo::create_game_activity(&mut conn, &game_activity).await;

            Ok(Json(GenericMessage::new("game proof publicized")))
        } else {
            Err((
                StatusCode::UNPROCESSABLE_ENTITY,
                "Invalid play proof".to_owned(),
            ))
        }
    } else {
        Ok(Json(GenericMessage::new("game proof publicized")))
    }
}
