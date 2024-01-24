use std::sync::Arc;

use ark_db::DBPool;
use ark_web3::json_rpcs;
use chaindexing::{Chain, Chaindexing, Repo};

pub fn start(db_pool: Arc<DBPool>) {
    tokio::spawn(async {
        let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&ark_db::url()))
            .with_initial_state(db_pool)
            .add_chain(Chain::Dev, &json_rpcs::get_local_url())
            .add_contract(coinflip_contracts::coinflip::get())
            .add_contract(ark_contracts::wallets::get());

        Chaindexing::index_states(&config).await.unwrap();
    });
}
