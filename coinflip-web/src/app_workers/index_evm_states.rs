use chaindexing::{Chain, Chaindexing, Repo};
use coinflip::CoinflipContract;

use tokio::task;

pub fn start() {
    task::spawn(async {
        dotenvy::dotenv().ok();
        let coinflip_contract = CoinflipContract::get();

        let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&ark_db::url()))
            .add_chain(
                Chain::Dev,
                &std::env::var("LOCAL_JSON_RPC_URL").expect("LOCAL_JSON_RPC_URL must be set"),
            )
            .add_contract(coinflip_contract)
            .reset(13);

        Chaindexing::index_states(&config).await.unwrap();
    });
}
