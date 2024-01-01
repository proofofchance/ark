// TODO: rename to index_contracts
use std::sync::Arc;

use ark_db::DBPool;
use ark_web3::get_local_json_rpc_url;
use chaindexing::{Chain, Chaindexing, Repo};

pub fn start(db_pool: Arc<DBPool>) {
    tokio::spawn(async {
        let config = chaindexing::Config::new(
            chaindexing::PostgresRepo::new(&ark_db::url()),
            Some(db_pool),
        )
        .add_chain(Chain::Dev, &get_local_json_rpc_url())
        .add_contract(coinflip_contracts::contract::get());

        Chaindexing::index_states(&config).await.unwrap();
    });
}
