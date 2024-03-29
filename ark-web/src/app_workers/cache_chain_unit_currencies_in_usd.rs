use std::sync::Arc;
use std::time::Duration;

use ark_db::DBPool;
use ark_web3::chains::{ChainId, UnsavedChainCurrency};
use strum::IntoEnumIterator;

use ark_web3::chains;

use tokio::time::interval;

const TWENTY_MINUTES: u64 = 20 * 60;

pub fn start(pool: Arc<DBPool>) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(TWENTY_MINUTES));
        let pool = pool.clone();
        let mut conn = pool.get().await.unwrap();

        loop {
            let test_net_chains = chains::get_test_nets();
            let chains = ChainId::iter().filter(|c| !test_net_chains.contains(c));

            let chain_currency_symbols: Vec<_> = chains.map(|c| c.get_currency_symbol()).collect();

            if let Ok(unit_prices_in_usd) =
                crypto_compare::get_unit_prices_in_usd(&chain_currency_symbols).await
            {
                let mut chain_currencies: Vec<_> = unit_prices_in_usd
                    .iter()
                    .map(|(currency_symbol, unit_usd_price)| {
                        let chain = ChainId::from_currency_symbol(currency_symbol);

                        UnsavedChainCurrency::new(chain, currency_symbol, *unit_usd_price)
                    })
                    .collect();

                let eth_unit_usd_price =
                    *unit_prices_in_usd.get(ChainId::Ethereum.get_currency_symbol()).unwrap();

                let local_chain_currencies = [
                    UnsavedChainCurrency::new(ChainId::Local, "LocalETH", eth_unit_usd_price),
                    UnsavedChainCurrency::new(ChainId::LocalAlt, "LocalAltETH", eth_unit_usd_price),
                ];
                chain_currencies.extend(local_chain_currencies);

                let testnet_currencies = [UnsavedChainCurrency::new(
                    ChainId::Sepolia,
                    "SepoliaETH",
                    eth_unit_usd_price,
                )];
                chain_currencies.extend(testnet_currencies);

                ark_repo::create_or_update_chain_currencies(&mut conn, &chain_currencies).await;
            }

            interval.tick().await;
        }
    });
}
