use std::time::Duration;

use ark_web_app::AppState;

use axum::error_handling::HandleErrorLayer;
use axum::routing::get;
use axum::BoxError;
use axum::{http::HeaderValue, Router};

use http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE,
};
use http::StatusCode;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::handlers::wallet_handler;

pub struct AppRouter {
    pub routes: Router<AppState>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self {
            routes: Router::new()
                .merge(Self::ark_routes())
                .layer(Self::cors_layer())
                .merge(Self::coinflip_routes())
                .layer(Self::cors_layer())
                .layer(
                    ServiceBuilder::new()
                        .layer(HandleErrorLayer::new(|err: BoxError| async move {
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                format!("Unhandled error: {}", err),
                            )
                        }))
                        .layer(BufferLayer::new(1024))
                        .layer(RateLimitLayer::new(4, Duration::from_secs(1))),
                )
                .layer(TraceLayer::new_for_http()),
        }
    }

    fn ark_routes() -> Router<AppState> {
        Router::new().route(
            "/wallets/:public_address/:chain_id",
            get(wallet_handler::get_wallet),
        )
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
                ACCESS_CONTROL_ALLOW_ORIGIN,
                AUTHORIZATION,
                CONTENT_TYPE,
            ])
    }
}
