// TODO: rename to index_contracts
use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::{Chain, Chaindexing, Repo};

use tokio::task;

pub fn start(db_pool: Arc<DBPool>) {
    task::spawn(async {
        dotenvy::dotenv().ok();

        let config = chaindexing::Config::new(
            chaindexing::PostgresRepo::new(&ark_db::url()),
            Some(db_pool),
        )
        .add_chain(
            Chain::Dev,
            &std::env::var("LOCAL_JSON_RPC_URL").expect("LOCAL_JSON_RPC_URL must be set"),
        )
        .add_contract(coinflip_contracts::contract::get());

        Chaindexing::index_states(&config).await.unwrap();
    });
}
