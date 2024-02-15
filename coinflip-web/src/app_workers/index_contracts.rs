use std::sync::Arc;

use ark_db::DBPool;
use ark_web3::json_rpcs;
use chaindexing::{Chain, Chaindexing, Repo};

pub fn start(db_pool: Arc<DBPool>) {
    tokio::spawn(async {
        let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&ark_db::url()))
            .with_ingestion_rate_ms(18_000)
            .with_initial_state(db_pool)
            .add_contract(coinflip_contracts::coinflip::get())
            .add_contract(ark_contracts::wallets::get())
            .reset(21)
            .add_reset_query("DELETE FROM coinflip_game_activities");

        let current_environment = ark::environments::current();

        let config = if current_environment.is_local() {
            config.add_chain(Chain::Dev, &json_rpcs::get_local_url())
        } else if current_environment.is_production() {
            config.add_chain(Chain::Sepolia, &json_rpcs::get_sepolia_url())
            // .add_chain(Chain::Mainnet, &json_rpcs::get_ethereum_url())
            // .add_chain(Chain::Polygon, &json_rpcs::get_polygon_url())
        } else {
            config
        };

        dbg!("Got to index contracts");
        Chaindexing::index_states(&config).await.unwrap();
    });
}
