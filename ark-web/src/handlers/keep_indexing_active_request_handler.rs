use ark_web_common::AppState;
use axum::extract::State;

use crate::handlers;

pub async fn refresh(State(app_state): State<AppState>) -> Result<(), handlers::Error> {
    app_state.keep_chaindexing_node_active_request.refresh().await;

    Ok(())
}
