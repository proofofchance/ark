// TODO: rename to index_contracts
use std::{collections::HashMap, sync::Arc, time::Duration};

use ark_db::DBPool;
use ark_web3::{get_ark_wallet, get_local_json_rpc_url};
use coinflip::GameStatus;
use coinflip_contracts::contract::get_coinflip_contract_address;
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

            let get_games_params = GetGamesParams::new()
                .reject_game_status(GameStatus::Expired)
                .filter_proofs_not_uploaded();

            let games = coinflip_repo::get_games(&mut conn, &get_games_params).await;
            let (game_ids, chain_ids): (Vec<_>, Vec<_>) =
                games.clone().iter().map(|g| (g.id, g.chain_id)).unzip();

            let game_plays =
                coinflip_repo::get_all_game_plays_with_proofs(&mut conn, &game_ids, &chain_ids)
                    .await;

            let play_ids_and_proofs_per_game =
                game_plays.iter().fold(HashMap::new(), |mut play_proofs_per_game, game_play| {
                    let game_id = game_play.game_id;
                    let chain_id = game_play.chain_id;
                    let game_and_chain_id = (game_id, chain_id);

                    let game_play_id = game_play.id;
                    let game_play_proof = game_play.play_proof.clone().unwrap();

                    match play_proofs_per_game.get(&game_and_chain_id) {
                        None => {
                            play_proofs_per_game.insert(
                                game_and_chain_id,
                                (vec![game_play_id as u16], vec![game_play_proof]),
                            );
                        }
                        Some((game_play_ids, play_proofs)) => {
                            let mut new_game_play_ids = game_play_ids.clone();
                            new_game_play_ids.push(game_play_id as u16);

                            let mut new_play_proofs = play_proofs.clone();
                            new_play_proofs.push(game_play_proof);

                            play_proofs_per_game
                                .insert(game_and_chain_id, (new_game_play_ids, new_play_proofs));
                        }
                    }

                    play_proofs_per_game
                });

            let games_by_id_and_chain_id =
                games.iter().fold(HashMap::new(), |mut games_by_id_and_chain_id, game| {
                    games_by_id_and_chain_id.insert((game.id, game.chain_id), game);
                    games_by_id_and_chain_id
                });

            for ((game_id, chain_id), (game_play_ids, proofs)) in
                play_ids_and_proofs_per_game.iter()
            {
                let game = games_by_id_and_chain_id.get(&(*game_id, *chain_id)).unwrap();
                if game.has_all_proofs_uploaded(proofs.len()) {
                    if upload_proofs_and_credit_winners(
                        *game_id as u64,
                        *chain_id as u64,
                        game_play_ids,
                        proofs,
                    )
                    .await
                    .is_ok()
                    {
                        coinflip_repo::record_proofs_uploaded(&mut conn, game.id, game.chain_id)
                            .await
                    }
                }
            }

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

async fn upload_proofs_and_credit_winners(
    game_id: u64,
    chain_id: u64,
    game_play_ids: &Vec<u16>,
    proofs: &Vec<String>,
) -> Result<(), String> {
    let provider = Provider::<Http>::try_from(&get_local_json_rpc_url()).unwrap();

    let wallet = get_ark_wallet(chain_id);
    let client = SignerMiddleware::new(provider, wallet);
    let client = Arc::new(client);

    let coinflip_contract_address: Address = get_coinflip_contract_address().parse().unwrap();
    let coinflip_contract = CoinflipContract::new(coinflip_contract_address, client);

    match coinflip_contract
        .upload_proofs_and_credit_winners(
            U256::from(game_id),
            game_play_ids.clone(),
            proofs.clone(),
        )
        .send()
        .await
    {
        Ok(data) => {
            dbg!(data);
            Ok(())
        }
        Err(err) => {
            dbg!(err);
            Err("Upload Error".to_owned())
        }
    }
}
