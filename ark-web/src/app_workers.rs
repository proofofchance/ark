// use std::{collections::HashMap, vec};

// use chaindexer::{Chain, Chaindexer, Chains, Contract, Event, EventHandler, JsonRpcUrl, Repo};
// use guessbucket_db::DB;
// use tokio::task;

// pub struct AppWorkers;

// impl AppWorkers {
//     pub fn start() {
//         Self::start_chain_indexer_worker()
//     }

//     fn start_chain_indexer_worker() {
//         task::spawn(async {
//             let config = chaindexer::Config::new(chaindexer::PostgresRepo::new(&DB::url()).await, Self::chains(), vec![
//                 Contract::new("BoredApeYachtClub")
//                 .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
//                 .add_address(
//                     "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
//                     &Chain::Mainnet,
//                     17773490,
//                 ),
//                 Contract::new("Doodles")
//                 .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
//                 .add_address(
//                     "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e",
//                     &Chain::Mainnet,
//                     17769635,
//                 ),
//             ]);

//             Chaindexer::index_states(&config).await.unwrap();
//         });
//     }

//     fn chains() -> Chains {
//         dotenvy::dotenv().ok();

//         HashMap::from([(
//             Chain::Mainnet,
//             JsonRpcUrl(
//                 std::env::var("MAINNET_JSON_RPC_URL").expect("MAINNET_JSON_RPC_URL must be set"),
//             ),
//         )])
//     }
// }

// struct TransferEventHandler;

// impl EventHandler for TransferEventHandler {
//     fn handle_event(_event: Event) {}
// }
