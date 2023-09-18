use ark_web::{AppRouter, AppServerConfig};
use ark_web_app::AppState;
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

    // Start Server
    let config = AppServerConfig::new();

    tracing::info!("Starting server at http://{}:{}/", config.host, config.port);

    axum::Server::bind(&config.socket_address())
        .serve(
            AppRouter::new()
                .routes
                .with_state(AppState {})
                .into_make_service(),
        )
        .await
        .unwrap();
}
