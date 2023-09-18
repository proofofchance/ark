use axum::Json;

use crate::handlers;

pub async fn get_games() -> Result<Json<Vec<String>>, handlers::Error> {
    Ok(Json(vec![]))
}
