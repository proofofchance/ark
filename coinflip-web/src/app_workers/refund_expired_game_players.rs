use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

use ark_db::DBPool;
use ark_web3::{json_rpcs, wallets};
use coinflip_contracts::coinflip::CoinflipContractAddress;
use coinflip_repo::GetGamesParams;
use tokio::time::{interval, sleep};

const ONE_MINUTE: u64 = 1 * 60;

pub fn start(pool: Arc<DBPool>) {
    tokio::spawn(async move {
        let mut has_once_waited_for_chaindexing_setup = false;
        const CHAINDEXING_SETUP_GRACE_PERIOD_SECS: u64 = 1 * 60;

        let mut interval = interval(Duration::from_secs(ONE_MINUTE));

        let pool = pool.clone();
        let mut conn = pool.get().await.unwrap();

        loop {
            if !has_once_waited_for_chaindexing_setup {
                sleep(Duration::from_secs(CHAINDEXING_SETUP_GRACE_PERIOD_SECS)).await;
                has_once_waited_for_chaindexing_setup = true;
            }

            let get_games_params = GetGamesParams::new().expired().not_refunded();

            let games = coinflip_repo::get_games(&mut conn, &get_games_params).await;
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

            let _result = refund_expired_game_players_for_all_games(game_ids_by_chain_id).await;

            interval.tick().await;
        }
    });
}

use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};

abigen!(
    CoinflipContract,
    "../orisirisi/libs/coinflip-contracts/deployments/localhost/Coinflip.json"
);

async fn refund_expired_game_players_for_all_games(
    game_ids_by_chain_id: HashMap<i64, Vec<i64>>,
) -> Result<(), String> {
    for (chain_id, game_ids) in game_ids_by_chain_id.iter() {
        let chain_id = &<i64 as Into<ark_web3::chains::Chain>>::into(*chain_id);
        let provider = Provider::<Http>::try_from(&json_rpcs::get_url(chain_id.into())).unwrap();
        let wallet = wallets::get(chain_id);
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        let coinflip_contract_address: Address =
            CoinflipContractAddress::get(chain_id).parse().unwrap();
        let coinflip_contract = CoinflipContract::new(coinflip_contract_address, client);
        let game_ids: Vec<_> = game_ids.iter().map(|game_id| U256::from(*game_id as u64)).collect();

        coinflip_contract
            .refund_expired_game_players_for_all_games(game_ids)
            .send()
            .await
            .map_err(|err| {
                dbg!(err);
                "Upload Error".to_owned()
            })?;
    }

    Ok(())
}