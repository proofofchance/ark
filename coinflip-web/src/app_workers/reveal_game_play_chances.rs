use std::{collections::HashMap, sync::Arc, time::Duration};

use ark_db::DBPool;
use ark_web3::{json_rpcs, wallets};
use coinflip::GamePlay;
use coinflip_contracts::contract::CoinflipContractAddress;
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

            let get_games_params = GetGamesParams::new().not_expired().only_incomplete();

            let games = coinflip_repo::get_games(&mut conn, &get_games_params).await;
            let (game_ids, chain_ids): (Vec<_>, Vec<_>) =
                games.clone().iter().map(|g| (g.id, g.chain_id)).unzip();

            let game_plays =
                coinflip_repo::get_all_game_plays_with_proofs(&mut conn, &game_ids, &chain_ids)
                    .await;

            let play_ids_and_chance_and_salts_per_game = game_plays.iter().fold(
                HashMap::new(),
                |mut play_ids_and_chance_and_salts_per_game, game_play| {
                    let game_id = game_play.game_id;
                    let chain_id = game_play.chain_id;
                    let game_and_chain_id = (game_id, chain_id);

                    let game_play_id = game_play.id;

                    let players_chance_and_salt: Bytes = GamePlay::get_chance_and_salt_bytes(
                        &game_play.chance_and_salt.clone().unwrap(),
                    )
                    .into();

                    match play_ids_and_chance_and_salts_per_game.get(&game_and_chain_id) {
                        None => {
                            play_ids_and_chance_and_salts_per_game.insert(
                                game_and_chain_id,
                                (vec![game_play_id as u16], vec![players_chance_and_salt]),
                            );
                        }
                        Some((game_play_ids, chance_and_salts)) => {
                            let mut new_game_play_ids = game_play_ids.clone();
                            new_game_play_ids.push(game_play_id as u16);

                            let mut new_chance_and_salts = chance_and_salts.clone();
                            new_chance_and_salts.push(players_chance_and_salt);

                            play_ids_and_chance_and_salts_per_game.insert(
                                game_and_chain_id,
                                (new_game_play_ids, new_chance_and_salts),
                            );
                        }
                    }

                    play_ids_and_chance_and_salts_per_game
                },
            );

            let games_by_id_and_chain_id =
                games.iter().fold(HashMap::new(), |mut games_by_id_and_chain_id, game| {
                    games_by_id_and_chain_id.insert((game.id, game.chain_id), game);
                    games_by_id_and_chain_id
                });

            for ((game_id, chain_id), (game_play_ids, chance_and_salts)) in
                play_ids_and_chance_and_salts_per_game.iter()
            {
                let game = games_by_id_and_chain_id.get(&(*game_id, *chain_id)).unwrap();
                if game.has_all_chances_uploaded(chance_and_salts.len()) {
                    let _result = reveal_chances_and_credit_winners(
                        *game_id as u64,
                        *chain_id as u64,
                        game_play_ids,
                        chance_and_salts,
                    )
                    .await;
                }
            }

            interval.tick().await;
        }
    });
}

use ethers::contract::abigen;
use ethers::middleware::SignerMiddleware;
use ethers::providers::{Http, Provider};
use ethers::types::{Address, Bytes, U256};

abigen!(
    CoinflipContract,
    "../orisirisi/libs/coinflip-contracts/deployments/localhost/Coinflip.json"
);

async fn reveal_chances_and_credit_winners(
    game_id: u64,
    chain_id: u64,
    game_play_ids: &Vec<u16>,
    chance_and_salts: &Vec<Bytes>,
) -> Result<(), String> {
    let chain_id = &<u64 as Into<ark_web3::chains::Chain>>::into(chain_id);
    let provider = Provider::<Http>::try_from(&json_rpcs::get_url(chain_id.into())).unwrap();

    let wallet = wallets::get(chain_id.into());
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let coinflip_contract_address: Address =
        CoinflipContractAddress::get(&chain_id).parse().unwrap();
    let coinflip_contract = CoinflipContract::new(coinflip_contract_address, client);

    coinflip_contract
        .reveal_chances_and_credit_winners(
            U256::from(game_id),
            game_play_ids.clone(),
            chance_and_salts.clone(),
        )
        .send()
        .await
        .map_err(|err| {
            dbg!(err);
            "Upload Error".to_owned()
        })?;

    Ok(())
}
