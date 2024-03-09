use std::sync::Arc;

use ark_db::DBPool;

use ark_utils::ethers::convert_wei_to_ether;
use chaindexing::{utils::address_to_string, ContractState, EventContext, EventHandler};

use super::states::Wallet;

pub struct CreditWalletEventHandler;

#[async_trait::async_trait]
impl EventHandler for CreditWalletEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let owner_address =
            address_to_string(&event_params.get("owner").unwrap().clone().into_address().unwrap())
                .to_lowercase();
        let credit_amount = convert_wei_to_ether(
            &event_params.get("amount").unwrap().clone().into_uint().unwrap().to_string(),
        );

        let initial_wallet = Wallet::read_one(
            [("owner_address".to_string(), owner_address.to_string())].into(),
            &event_context,
        )
        .await;

        let initial_balance = get_initial_balance(&initial_wallet).await;
        let new_balance = initial_balance + credit_amount;

        create_or_update_wallet_balance(
            &initial_wallet,
            new_balance,
            owner_address,
            &event_context,
        )
        .await;
    }
}

pub struct DebitWalletEventHandler;

#[async_trait::async_trait]
impl EventHandler for DebitWalletEventHandler {
    type SharedState = Arc<DBPool>;

    async fn handle_event<'a>(&self, event_context: EventContext<'a, Self::SharedState>) {
        let event = &event_context.event;
        let event_params = event.get_params();

        let owner_address =
            address_to_string(&event_params.get("owner").unwrap().clone().into_address().unwrap())
                .to_lowercase();
        let debit_amount = convert_wei_to_ether(
            &event_params.get("amount").unwrap().clone().into_uint().unwrap().to_string(),
        );

        let initial_wallet = Wallet::read_one(
            [("owner_address".to_string(), owner_address.to_string())].into(),
            &event_context,
        )
        .await;

        let initial_balance = get_initial_balance(&initial_wallet).await;
        let new_balance = initial_balance - debit_amount;

        create_or_update_wallet_balance(
            &initial_wallet,
            new_balance,
            owner_address,
            &event_context,
        )
        .await;
    }
}

async fn get_initial_balance<'a>(initial_wallet: &Option<Wallet>) -> f64 {
    initial_wallet
        .as_ref()
        .map(|w| w.balance.parse::<f64>().unwrap())
        .unwrap_or(0.0)
}

async fn create_or_update_wallet_balance<'a>(
    initial_wallet: &Option<Wallet>,
    new_balance: f64,
    owner_address: String,
    event_context: &EventContext<'a, Arc<DBPool>>,
) {
    if initial_wallet.is_none() {
        Wallet {
            balance: new_balance.to_string(),
            owner_address,
        }
        .create(&event_context)
        .await;
    } else {
        initial_wallet
            .clone()
            .unwrap()
            .update(
                [("balance".to_string(), new_balance.to_string())].into(),
                &event_context,
            )
            .await;
    }
}
