use std::sync::Arc;

use ark_db::DBPool;
use ark_web3::json_rpcs;
use chaindexing::{Chain, ChainId, Chaindexing, KeepNodeActiveRequest, OptimizationConfig, Repo};

pub fn start(pool: Arc<DBPool>, keep_chaindexing_node_active_request: KeepNodeActiveRequest) {
    tokio::spawn(async move {
        let optimization_config = OptimizationConfig {
            keep_node_active_request: keep_chaindexing_node_active_request,
            optimize_after_in_secs: 12 * 60,
        };

        let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&ark_db::url()))
            .with_ingestion_rate_ms(6_000)
            .with_initial_state(pool)
            .add_contract(coinflip_contracts::coinflip::get())
            .add_contract(ark_contracts::wallets::get())
            .reset(get_reset_count())
            .add_reset_query("DELETE FROM coinflip_game_activities")
            .enable_optimization(&optimization_config);

        let current_environment = ark::environments::current();

        let config = if current_environment.is_local() {
            config.add_chain(Chain::new(ChainId::Dev, &json_rpcs::get_local_url()))
        } else if current_environment.is_production() {
            config
                .add_chain(Chain::new(ChainId::Sepolia, &json_rpcs::get_sepolia_url()))
                .add_chain(Chain::new(ChainId::Polygon, &json_rpcs::get_polygon_url()))
            // .add_chain(Chain::new(ChainId::Mainnet, &json_rpcs::get_ethereum_url()))
        } else {
            config
        };

        Chaindexing::index_states(&config).await.unwrap();
    });
}

pub fn get_reset_count() -> u8 {
    dotenvy::dotenv().ok();

    std::env::var("CHAINDEXING_RESET_COUNT")
        .map(|rc| rc.parse::<u8>().expect("CHAINDEXING_RESET_COUNT must be of type u8"))
        .unwrap_or(0)
}
