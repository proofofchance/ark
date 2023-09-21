use std::{collections::HashMap, vec};

use ark_db::DB;
use chaindexing::{Chain, Chaindexing, Chains, Contract, Event, EventHandler, Repo};
use tokio::task;

pub struct IndexEvmStates;

impl IndexEvmStates {
    pub fn start() {
        task::spawn(async {
            let config = chaindexing::Config::new(chaindexing::PostgresRepo::new(&DB::url()), Self::chains(), vec![
                Contract::new("BoredApeYachtClub")
                .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
                .add_address(
                    "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
                    &Chain::Mainnet,
                    17773490,
                ),
                Contract::new("Doodles")
                .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
                .add_address(
                    "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e",
                    &Chain::Mainnet,
                    17769635,
                ),
            ]);

            Chaindexing::index_states(&config).await.unwrap();
        });
    }

    fn chains() -> Chains {
        dotenvy::dotenv().ok();

        HashMap::from([(
            Chain::Mainnet,
            std::env::var("MAINNET_JSON_RPC_URL").expect("MAINNET_JSON_RPC_URL must be set"),
        )])
    }
}

struct TransferEventHandler;

#[async_trait::async_trait]
impl EventHandler for TransferEventHandler {
    async fn handle_event(&self, _event: Event) {}
}
