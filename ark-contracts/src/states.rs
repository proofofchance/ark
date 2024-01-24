use chaindexing::{ContractState, ContractStateMigrations};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub owner_address: String,
    pub balance: String,
}

impl ContractState for Wallet {
    fn table_name() -> &'static str {
        "ark_wallets"
    }
}

pub struct WalletMigrations;

impl ContractStateMigrations for WalletMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS ark_wallets (
                id BIGSERIAL PRIMARY KEY,
                owner_address VARCHAR NOT NULL,
                balance VARCHAR NOT NULL,
            )",
        ]
    }
}
