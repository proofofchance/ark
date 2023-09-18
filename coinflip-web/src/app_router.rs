use ark_web_app::AppState;
use axum::{routing::get, Router};

use crate::handlers::game_handler;

pub struct AppRouter {
    pub routes: Router<AppState>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new().merge(Self::game_routes()),
        }
    }

    fn game_routes() -> Router<AppState> {
        Router::new().nest(
            "/games",
            Router::new().route("/", get(game_handler::get_games)),
        )
    }
}
