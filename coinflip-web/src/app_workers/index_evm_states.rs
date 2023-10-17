use std::collections::HashMap;

use ark_db::DB;
use chaindexing::{
    Chain, Chaindexing, Chains, Contract, ContractState, ContractStateMigrations, EventContext,
    EventHandler, Repo,
};
use serde::{Deserialize, Serialize};
use tokio::task;

pub struct IndexEvmStates;

impl IndexEvmStates {
    pub fn start() {
        task::spawn(async {
            let bayc_contract =  Contract::new("BoredApeYachtClub")
            .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
            .add_state_migrations(NftStateMigrations)
            .add_address(
                "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D",
                &Chain::Mainnet,
                17773490,
            );
            let doodles_contract =  Contract::new("Doodles")
            .add_event("event Transfer(address indexed from, address indexed to, uint256 indexed tokenId)", TransferEventHandler)
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct NftState {
    token_id: i32,
    contract_address: String,
    owner_address: String,
}

impl ContractState for NftState {
    fn table_name() -> &'static str {
        "nft_states"
    }
}

struct NftStateMigrations;

impl ContractStateMigrations for NftStateMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS nft_states (
                token_id INTEGER NOT NULL,
                contract_address TEXT NOT NULL,
                owner_address TEXT NOT NULL
            )",
        ]
    }
}

struct TransferEventHandler;

#[async_trait::async_trait]
impl EventHandler for TransferEventHandler {
    async fn handle_event<'a>(&self, event_context: EventContext<'a>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let from = event_params.get("from").unwrap().clone().into_address().unwrap();
        let to = event_params.get("to").unwrap().clone().into_address().unwrap();
        let token_id = event_params.get("tokenId").unwrap().clone().into_uint().unwrap();

        if let Some(nft_state) = NftState::read_one(
            [
                ("token_id".to_owned(), token_id.to_string()),
                ("owner_address".to_owned(), from.to_string()),
            ]
            .into(),
            &event_context,
        )
        .await
        {
            let updates = [("owner_address".to_string(), to.to_string())];

            nft_state.update(updates.into(), &event_context).await;
        } else {
            NftState {
                token_id: token_id.as_u32() as i32,
                contract_address: event.contract_address.clone(),
                owner_address: to.to_string(),
            }
            .create(&event_context)
            .await;
        }
    }
}
