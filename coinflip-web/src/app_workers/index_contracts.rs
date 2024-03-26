use std::sync::Arc;

use ark_db::DBPool;
use ark_web3::json_rpcs;
use chaindexing::{Chain, ChainId, Chaindexing, KeepNodeActiveRequest, OptimizationConfig, Repo};

// TODO: Move to ark-level
pub fn start(pool: Arc<DBPool>, keep_chaindexing_node_active_request: KeepNodeActiveRequest) {
    tokio::spawn(async move {
        let optimization_config = OptimizationConfig {
            keep_node_active_request: keep_chaindexing_node_active_request,
            optimize_after_in_secs: 6 * 60,
        };

        let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&ark_db::url()))
            .with_ingestion_rate_ms(6_000)
            .with_initial_state(pool)
            .add_contract(coinflip_contracts::coinflip::get())
            .add_contract(ark_contracts::wallets::get())
            .reset(get_reset_count())
            .add_reset_query("DELETE FROM coinflip_game_activities")
            .add_reset_query("DELETE FROM ark_total_paid_out_reports")
            .enable_optimization(&optimization_config)
            .with_pruning();

        let current_environment = ark::environments::current();

        let config = if current_environment.is_local() {
            config.add_chain(Chain::new(ChainId::Dev, &json_rpcs::get_local_url()))
        } else if current_environment.is_production() {
            config
                .add_chain(Chain::new(ChainId::Sepolia, &json_rpcs::get_sepolia_url()))
                .add_chain(Chain::new(ChainId::Polygon, &json_rpcs::get_polygon_url()))
                .add_chain(Chain::new(ChainId::Mainnet, &json_rpcs::get_ethereum_url()))
        } else {
            config
        };

        Chaindexing::index_states(&config).await.unwrap();
    });
}

pub fn get_reset_count() -> u64 {
    dotenvy::dotenv().ok();

    std::env::var("CHAINDEXING_RESET_COUNT")
        .map(|rc| rc.parse::<u64>().expect("CHAINDEXING_RESET_COUNT must be of type u64"))
        .unwrap_or(0)
}
