use std::sync::Arc;
use std::time::Duration;

use ark_db::DBPool;
use ark_repo::Repo;
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

            for chain in chains {
                let chain_currency = chain.get_currency();
                let unit_usd_price =
                    crypto_compare::get_unit_price_in_usd(chain_currency).await.unwrap();

                Repo::create_or_update_chain_currency(
                    &mut conn,
                    chain as usize,
                    chain_currency,
                    unit_usd_price,
                )
                .await;
            }

            interval.tick().await;
        }
    });
}
