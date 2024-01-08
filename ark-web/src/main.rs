use std::sync::Arc;

use ark_web::{AppRouter, AppServerConfig};
use ark_web_app::AppState;
use coinflip_web::app_workers::{
    cache_chain_unit_currencies_in_usd, index_evm_states, reveal_game_play_chances,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ark_web=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = Arc::new(ark_db::get_pool().await);

    // Start Workers
    index_evm_states::start(db_pool.clone());
    cache_chain_unit_currencies_in_usd::start(db_pool.clone());
    reveal_game_play_chances::start(db_pool.clone());

    // Start Server
    let config = AppServerConfig::new();

    tracing::info!("Starting server at http://{}:{}/", config.host, config.port);

    let listener = tokio::net::TcpListener::bind(&config.socket_address()).await.unwrap();

    axum::serve(
        listener,
        AppRouter::new().routes.with_state(AppState::new(db_pool)).into_make_service(),
    )
    .await
    .unwrap();
}
