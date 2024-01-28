use ark_web_app::AppState;
use axum::{
    routing::{get, put},
    Router,
};

use crate::handlers::{game_activity_handler, game_handler, game_play_handler};

pub struct AppRouter {
    pub routes: Router<AppState>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new()
                .merge(Self::game_routes())
                .merge(Self::game_play_routes())
                .merge(Self::game_activty_routes()),
        }
    }

    fn game_routes() -> Router<AppState> {
        Router::new().nest(
            "/games",
            Router::new()
                .route("/", get(game_handler::get_games))
                .route("/:id/:chain_id", get(game_handler::get_game))
                .route(
                    "/:id/:chain_id/activities",
                    get(game_activity_handler::get_game_activities),
                ),
        )
    }

    fn game_play_routes() -> Router<AppState> {
        Router::new().nest(
            "/game_plays/:game_id/:chain_id",
            Router::new().route("/my_game_play", put(game_play_handler::update_my_game_play)),
        )
    }

    fn game_activty_routes() -> Router<AppState> {
        Router::new().nest(
            "/game_activities",
            Router::new().route(
                "/:game_status/:player_address",
                get(game_activity_handler::get_all_game_activites),
            ),
        )
    }
}
