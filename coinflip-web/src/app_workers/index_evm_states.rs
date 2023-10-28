use std::collections::HashMap;

use ark_db::DB;
use chaindexing::{Chain, Chaindexing, Chains, Repo};
use coinflip::CoinflipContract;

use tokio::task;

pub struct IndexEvmStates;

impl IndexEvmStates {
    pub fn start() {
        task::spawn(async {
            let coinflip_contract = CoinflipContract::get();

            let config = chaindexing::Config::new(
                chaindexing::PostgresRepo::new(&DB::url()),
                Self::chains(),
            )
            .add_contract(coinflip_contract)
            .reset(11);

            Chaindexing::index_states(&config).await.unwrap();
        });
    }

    fn chains() -> Chains {
        dotenvy::dotenv().ok();

        HashMap::from([(
            Chain::Dev,
            std::env::var("LOCAL_JSON_RPC_URL").expect("LOCAL_JSON_RPC_URL must be set"),
        )])
    }
}
