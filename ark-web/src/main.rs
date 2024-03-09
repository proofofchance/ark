use std::sync::Arc;

use ark_web::{AppRouter, AppServerConfig};
use ark_web_common::AppState;

use ark_web::app_workers::cache_chain_unit_currencies_in_usd;
use chaindexing::KeepNodeActiveRequest;
use coinflip_web::app_workers::{
    index_contracts, refund_expired_game_players, reveal_game_play_chances,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ark_web=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = Arc::new(ark_db::get_pool().await);
    let keep_chaindexing_node_active_request = KeepNodeActiveRequest::new(200_000);

    // Start Workers
    index_contracts::start(
        db_pool.clone(),
        keep_chaindexing_node_active_request.clone(),
    );
    cache_chain_unit_currencies_in_usd::start(db_pool.clone());
    reveal_game_play_chances::start(
        db_pool.clone(),
        keep_chaindexing_node_active_request.clone(),
    );
    refund_expired_game_players::start(
        db_pool.clone(),
        keep_chaindexing_node_active_request.clone(),
    );

    // Start Server
    let config = AppServerConfig::new();

    tracing::info!("Starting server at http://{}:{}/", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&config.socket_address()).await.unwrap();

    axum::serve(
        listener,
        AppRouter::new()
            .routes
            .with_state(AppState::new(
                db_pool,
                &keep_chaindexing_node_active_request,
            ))
            .into_make_service(),
    )
    .await
    .unwrap();
}
