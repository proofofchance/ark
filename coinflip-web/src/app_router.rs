use ark_web_app::AppState;
use axum::{routing::get, Router};
use http::{
    header::{ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue,
};
use tower_http::cors::{Any, CorsLayer};

use crate::handlers::{game_activity_handler, game_handler};

pub struct AppRouter {
    pub routes: Router<AppState>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new()
                .merge(Self::game_routes())
                .merge(Self::game_activty_routes())
                .layer(Self::cors_layer()),
        }
    }

    fn game_routes() -> Router<AppState> {
        Router::new().nest(
            "/games",
            Router::new()
                .route("/", get(game_handler::get_games))
                .route("/:id", get(game_handler::get_game)),
        )
    }

    fn game_activty_routes() -> Router<AppState> {
        Router::new().nest(
            "/game_activities",
            Router::new().route(
                "/ongoing/:player_address",
                get(game_activity_handler::get_ongoing_game_activities),
            ),
        )
    }

    fn cors_layer() -> CorsLayer {
        dotenvy::dotenv().ok();

        let frontend_origin =
            std::env::var("FRONTEND_ORIGIN").expect("FRONTEND_ORIGIN must be set");

        CorsLayer::new()
            .allow_origin(frontend_origin.parse::<HeaderValue>().unwrap())
            .allow_methods(Any)
            .allow_headers([
                ACCEPT,
                ACCESS_CONTROL_ALLOW_HEADERS,
                AUTHORIZATION,
                CONTENT_TYPE,
            ])
    }
}
