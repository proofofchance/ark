use std::sync::Arc;
use std::time::Duration;

use ark_db::DBPool;
use ark_repo::Repo;
use coinflip::chains::UnsavedChainCurrency;
use strum::IntoEnumIterator;

use coinflip::{chains, Chain};

use tokio::task;
use tokio::time::interval;

const TWENTY_MINUTES: u64 = 20 * 60;

pub fn start(pool: Arc<DBPool>) {
    task::spawn(async move {
        let mut interval = interval(Duration::from_secs(TWENTY_MINUTES));
        let pool = pool.clone();
        let mut conn = pool.get().await.unwrap();

        loop {
            let test_net_chains = chains::get_test_nets();
            let chains = Chain::iter().filter(|c| !test_net_chains.contains(c));

            let chain_currency_symbols: Vec<_> = chains.map(|c| c.get_currency_symbol()).collect();

            let unit_prices_in_usd =
                crypto_compare::get_unit_prices_in_usd(&chain_currency_symbols).await.unwrap();

            let chain_currencies: Vec<_> = unit_prices_in_usd
                .iter()
                .map(|(currency_symbol, unit_usd_price)| {
                    let chain = Chain::from_currency_symbol(currency_symbol);

                    UnsavedChainCurrency::new(chain, currency_symbol, *unit_usd_price)
                })
                .collect();

            Repo::create_or_update_chain_currencies(&mut conn, &chain_currencies).await;

            interval.tick().await;
        }
    });
}
