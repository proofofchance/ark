use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

use super::caching::RecentCache;
use ark_db::DBPool;
use ark_web3::chain_explorers::GasInfo;
use ark_web3::{chain_explorers, json_rpcs, wallets, CHAIN_AGNOSTIC_MAX_GAS_PRICE};
use chaindexing::KeepNodeActiveRequest;
use coinflip_repo::GetGamesParams;
use eyre::Result;
use tokio::time::{interval, sleep};
use tracing::info;

const WORKER_INTERVAL_MS: u64 = 10 * 60 * 1_000;

pub fn start(pool: Arc<DBPool>, keep_chaindexing_node_active_request: KeepNodeActiveRequest) {
    tokio::spawn(async move {
        let mut has_once_waited_for_chaindexing_setup = false;
        const CHAINDEXING_SETUP_GRACE_PERIOD_SECS: u64 = 1 * 60;

        let mut interval = interval(Duration::from_millis(WORKER_INTERVAL_MS));

        let pool = pool.clone();
        let mut conn = pool.get().await.unwrap();

        const TWENTY_MINS: u64 = 20 * 60;
        let mut cached_gas_infos = RecentCache::new(TWENTY_MINS);

        loop {
            if !has_once_waited_for_chaindexing_setup {
                sleep(Duration::from_secs(CHAINDEXING_SETUP_GRACE_PERIOD_SECS)).await;
                has_once_waited_for_chaindexing_setup = true;
            }

            info!("[RefundExpiredGamePlayers]: running...");

            let get_games_params = GetGamesParams::new().expired().not_refunded();

            let games = coinflip_repo::get_games(&mut conn, &get_games_params).await;

            info!(
                "[RefundExpiredGamePlayers]: Found {} games...",
                &games.len()
            );

            let game_ids_by_chain_id =
                games.iter().fold(HashMap::new(), |mut game_ids_by_chain_id, game| {
                    match game_ids_by_chain_id.get(&game.chain_id) {
                        None => {
                            game_ids_by_chain_id.insert(game.chain_id, vec![game.id]);
                        }
                        Some(game_ids) => {
                            let mut new_game_ids = game_ids.clone();
                            new_game_ids.push(game.id);
                            game_ids_by_chain_id.insert(game.chain_id, new_game_ids);
                        }
                    }

                    game_ids_by_chain_id
                });

            match refund_expired_game_players_for_all_games(
                game_ids_by_chain_id,
                &mut cached_gas_infos,
            )
            .await
            {
                Ok(()) => keep_chaindexing_node_active_request.refresh().await,
                Err(err) => {
                    cached_gas_infos.invalidate_all();
                    dbg!(err.to_string());
                }
            }

            interval.tick().await;
        }
    });
}

use ethers::contract::abigen;
use ethers::middleware::gas_escalator::{Frequency, LinearGasPrice};
use ethers::middleware::{GasEscalatorMiddleware, SignerMiddleware};
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};

use ark_web3::chains::ChainId;

abigen!(
    CoinflipContract,
    r#"[
        function refundExpiredGamePlayersForGames(uint[] memory gameIDs) external 
    ]"#,
);

async fn refund_expired_game_players_for_all_games(
    game_ids_by_chain_id: HashMap<i64, Vec<i64>>,
    cached_gas_infos: &mut RecentCache<ChainId, GasInfo>,
) -> Result<()> {
    for (chain_id, game_ids) in game_ids_by_chain_id.iter() {
        let escalator = {
            let every_min: u64 = 60 * 60;
            let max_price: Option<u64> = Some(CHAIN_AGNOSTIC_MAX_GAS_PRICE);

            let increase_by: u64 = 100;
            LinearGasPrice::new(increase_by, every_min, max_price)
        };

        let chain_id = &<u64 as Into<ChainId>>::into(*chain_id as u64);
        let provider = Provider::<Http>::try_from(&json_rpcs::get_url(chain_id.into())).unwrap();
        let provider = GasEscalatorMiddleware::new(provider, escalator, Frequency::PerBlock);

        let wallet = wallets::get(chain_id);
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        let coinflip_contract_address: Address =
            chain_id.get_contract_address("COINFLIP").parse().unwrap();
        let coinflip_contract = CoinflipContract::new(coinflip_contract_address, client);
        let game_ids: Vec<_> = game_ids.iter().map(|game_id| U256::from(*game_id as u64)).collect();

        let cached_gas_info = {
            if let Some(gas_info) = cached_gas_infos.get(chain_id) {
                gas_info.clone()
            } else {
                let gas_info = chain_explorers::get_gas_info(&chain_id).await?;
                cached_gas_infos.insert(*chain_id, gas_info.clone());
                gas_info
            }
        };
        coinflip_contract
            .refund_expired_game_players_for_games(game_ids)
            .gas_price(cached_gas_info.get_safe_price_wei())
            .send()
            .await?;
    }

    Ok(())
}
