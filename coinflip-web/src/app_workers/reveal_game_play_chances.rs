use std::{collections::HashMap, sync::Arc, time::Duration};

use ark_db::DBPool;
use ark_web3::{json_rpcs, wallets, CHAIN_AGNOSTIC_MAX_GAS_PRICE};
use chaindexing::KeepNodeActiveRequest;
use coinflip::GamePlay;
use coinflip_repo::GetGamesParams;
use tokio::time::{interval, sleep};

const WORKER_INTERVAL_MS: u64 = 1 * 60 * 1_000;

pub fn start(pool: Arc<DBPool>, keep_chaindexing_node_active_request: KeepNodeActiveRequest) {
    tokio::spawn(async move {
        let mut has_once_waited_for_chaindexing_setup = false;
        const CHAINDEXING_CATCH_UP_GRACE_PERIOD_SECS: u64 = 20 * 60;

        let mut interval = interval(Duration::from_millis(WORKER_INTERVAL_MS));

        let pool = pool.clone();
        let mut conn = pool.get().await.unwrap();

        loop {
            if !has_once_waited_for_chaindexing_setup {
                sleep(Duration::from_secs(CHAINDEXING_CATCH_UP_GRACE_PERIOD_SECS)).await;
                has_once_waited_for_chaindexing_setup = true;
            }

            let get_games_params = GetGamesParams::new().not_expired().only_incomplete();

            let games = coinflip_repo::get_games(&mut conn, &get_games_params).await;

            dbg!(&games);

            let (game_ids, chain_ids): (Vec<_>, Vec<_>) =
                games.clone().iter().map(|g| (g.id, g.chain_id)).unzip();

            let mut game_plays =
                coinflip_repo::get_all_game_plays_with_proofs(&mut conn, &game_ids, &chain_ids)
                    .await;

            dbg!(&game_plays);

            // Sort to ensure chances_and_salts are in the expected ascending order in terms of their ids
            game_plays.sort_by(|a, b| a.cmp(b));

            let chance_and_salts_per_game = game_plays.iter().fold(
                HashMap::new(),
                |mut chance_and_salts_per_game, game_play| {
                    let game_id = game_play.game_id;
                    let chain_id = game_play.chain_id;
                    let game_and_chain_id = (game_id, chain_id);

                    let players_chance_and_salt: Bytes = GamePlay::get_chance_and_salt_bytes(
                        &game_play.chance_and_salt.clone().unwrap(),
                    )
                    .into();

                    match chance_and_salts_per_game.get(&game_and_chain_id) {
                        None => {
                            chance_and_salts_per_game
                                .insert(game_and_chain_id, vec![players_chance_and_salt]);
                        }
                        Some(chance_and_salts) => {
                            let mut new_chance_and_salts = chance_and_salts.clone();
                            new_chance_and_salts.push(players_chance_and_salt);

                            chance_and_salts_per_game
                                .insert(game_and_chain_id, new_chance_and_salts);
                        }
                    }

                    chance_and_salts_per_game
                },
            );

            dbg!(&chance_and_salts_per_game);
            let games_by_id_and_chain_id =
                games.iter().fold(HashMap::new(), |mut games_by_id_and_chain_id, game| {
                    games_by_id_and_chain_id.insert((game.id, game.chain_id), game);
                    games_by_id_and_chain_id
                });

            dbg!(&games_by_id_and_chain_id);

            for ((game_id, chain_id), chance_and_salts) in chance_and_salts_per_game.iter() {
                dbg!((game_id, chain_id));
                let game = games_by_id_and_chain_id.get(&(*game_id, *chain_id)).unwrap();
                if game.has_all_chances_uploaded(chance_and_salts.len()) {
                    if reveal_chances_and_credit_winners(
                        *game_id as u64,
                        *chain_id as u64,
                        chance_and_salts,
                    )
                    .await
                    .is_ok()
                    {
                        keep_chaindexing_node_active_request.refresh().await;
                    }
                }
            }

            interval.tick().await;
        }
    });
}

use ethers::contract::abigen;
use ethers::middleware::gas_escalator::{Frequency, GeometricGasPrice};
use ethers::middleware::GasEscalatorMiddleware;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, Bytes, U256};

use ark_web3::chains::ChainId;

abigen!(
    CoinflipContract,
    r#"[
        function revealChancesAndCreditWinners(uint gameID, bytes[] calldata chanceAndSalts) external
    ]"#,
);

async fn reveal_chances_and_credit_winners(
    game_id: u64,
    chain_id: u64,
    chance_and_salts: &Vec<Bytes>,
) -> Result<(), String> {
    let chain_id = &<u64 as Into<ChainId>>::into(chain_id);
    let escalator = {
        let every_secs: u64 = 60;
        let max_price: Option<u64> = Some(CHAIN_AGNOSTIC_MAX_GAS_PRICE);

        let coefficient: f64 = match chain_id {
            ChainId::Polygon => 20_000.0,
            _ => 1.15,
        };

        GeometricGasPrice::new(coefficient, every_secs, max_price)
    };
    let provider = Provider::<Http>::try_from(&json_rpcs::get_url(chain_id.into())).unwrap();
    let provider = GasEscalatorMiddleware::new(provider, escalator, Frequency::PerBlock);

    let wallet = wallets::get(chain_id.into());
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let coinflip_contract_address: Address =
        chain_id.get_contract_address("COINFLIP").parse().unwrap();
    let coinflip_contract = CoinflipContract::new(coinflip_contract_address, client);

    coinflip_contract
        .reveal_chances_and_credit_winners(U256::from(game_id), chance_and_salts.clone())
        .send()
        .await
        .map_err(|err| {
            dbg!(err);
            "Upload Error".to_owned()
        })?;

    Ok(())
}
