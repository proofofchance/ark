use ark_web_app::AppState;
use axum::{http::HeaderValue, Router};
use http::header::{ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::{Any, CorsLayer};

pub struct AppRouter {
    pub routes: Router<AppState>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new()
                .merge(Self::coinflip_routes())
                .layer(Self::cors_layer()),
        }
    }

    fn coinflip_routes() -> Router<AppState> {
        Router::new().nest("/coinflip", coinflip_web::AppRouter::new().routes)
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
