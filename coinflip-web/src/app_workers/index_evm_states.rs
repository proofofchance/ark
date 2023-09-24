use std::collections::HashMap;

use ark_db::DB;
use chaindexing::{Chain, Chaindexing, Chains, Contract, ContractState, Event, EventHandler, Repo};
use tokio::task;

pub struct IndexEvmStates;

impl IndexEvmStates {
    pub fn start() {
        task::spawn(async {
            let bayc_contract =  Contract::new("BoredApeYachtClub")
            .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
            .add_event("event ApprovalForAll(address indexed owner, address indexed operator, bool approved)", ApprovalForAllEventHandler)
            .add_address(
                "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
                &Chain::Mainnet,
                17773490,
            );
            let doodles_contract =  Contract::new("Doodles")
            .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
            .add_event("event ApprovalForAll(address indexed owner, address indexed operator, bool approved)", ApprovalForAllEventHandler)
            .add_address(
                "0x8a90CAb2b38dba80c64b7734e58Ee1dB38B8992e",
                &Chain::Mainnet,
                17769635,
            );

            let config = chaindexing::Config::new(
                chaindexing::PostgresRepo::new(&DB::url()),
                Self::chains(),
            )
            .add_contract(bayc_contract)
            .add_contract(doodles_contract);

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

#[derive(Clone, Debug)]
enum CoinflipContractState {
    NftState(NftState),
    NftOperatorState(NftOperatorState),
}

impl ContractState for CoinflipContractState {}

#[derive(Clone, Debug)]
struct NftState;

impl ContractState for NftState {}

struct TransferEventHandler;

#[async_trait::async_trait]
impl EventHandler for TransferEventHandler {
    type State = CoinflipContractState;
    async fn handle_event(&self, _event: Event) -> Option<Vec<Self::State>> {
        dbg!("Calls Transfter Event Handler Event");

        None
    }
}

#[derive(Clone, Debug)]
struct NftOperatorState;

impl ContractState for NftOperatorState {}

struct ApprovalForAllEventHandler;

#[async_trait::async_trait]
impl EventHandler for ApprovalForAllEventHandler {
    type State = CoinflipContractState;
    async fn handle_event(&self, event: Event) -> Option<Vec<Self::State>> {
        dbg!(format!(
            "Calls Approval For ALL EVent Handler Event for {}",
            event.contract_name
        ));

        None
    }
}
