use std::collections::HashMap;

use ark_utils::floats;
use ark_web_app::AppState;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use coinflip_repo::{GetGamesParams, Repo};

use coinflip::{chains::ChainCurrency, Chain, Game, GamePlay, GameStatus};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::handlers;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameResponse {
    id: u64,
    chain_id: u32,
    expiry_timestamp: u64,
    creator_address: String,
    block_number: u64,
    status: GameStatus,
    wager: f64,
    wager_usd: f64,
    max_possible_win_usd: f64,
    players_left: u32,
    total_players_required: u32,
    is_in_play_phase: bool,
    unavailable_coin_side: Option<i32>,
    is_awaiting_my_play_proof: Option<bool>, // view_count: u64,
    my_game_play_id: Option<i32>,
}

impl GameResponse {
    fn new(game: &Game, chain_currency: &ChainCurrency) -> Self {
        let total_players_required = game.max_play_count as u32;

        let wager = game.get_wager_ether_unit();
        let wager_usd = chain_currency.convert_to_usd(wager);
        let wager_usd = floats::to_2dp(wager_usd);

        GameResponse {
            id: game.id as u64,
            chain_id: game.chain_id as u32,
            expiry_timestamp: game.expiry_timestamp as u64,
            creator_address: game.creator_address.clone(),
            block_number: game.block_number as u64,
            status: game.get_status(),
            wager,
            wager_usd,
            // TODO: Should be calculated from the number of heads and tails so far (whichever has most)
            // If no play yet, then it is total players required * wager usd
            max_possible_win_usd: total_players_required as f64 * wager_usd,
            players_left: game.get_players_left(),
            total_players_required,
            is_in_play_phase: game.is_in_play_phase(),
            is_awaiting_my_play_proof: None, // view_count: 0,
            unavailable_coin_side: game.unavailable_coin_side,
            my_game_play_id: None,
        }
    }

    fn maybe_set_is_awaiting_my_play_proof(
        mut self,
        game: &Game,
        maybe_game_play: &Option<GamePlay>,
    ) -> Self {
        self.is_awaiting_my_play_proof = if game.is_expired() || game.is_in_play_phase() {
            None
        } else {
            maybe_game_play
                .as_ref()
                .and_then(|game_play| Some(game_play.play_proof.is_none()))
        };

        self
    }

    fn maybe_set_my_game_play_id(mut self, maybe_game_play: &Option<GamePlay>) -> Self {
        if let Some(GamePlay { id, .. }) = maybe_game_play {
            self.my_game_play_id = Some(*id);
        }

        self
    }
}

pub async fn get_games(
    State(app_state): State<AppState>,
    query_params: Query<GetGamesParams>,
) -> Result<Json<Vec<GameResponse>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_all_games(&mut conn, &query_params).await;

    let chain_ids: Vec<_> = games.iter().map(|game| game.get_chain_id()).collect();

    let chain_currencies = Repo::get_chain_currencies(&mut conn, &chain_ids).await;
    let chain_currencies_by_chain_id = chain_currencies.iter().fold(
        HashMap::new(),
        |mut chain_currencies_by_chain_id, chain_currency| {
            chain_currencies_by_chain_id.insert(chain_currency.chain_id, chain_currency);

            if chain_currency.chain_id == (Chain::Ethereum as i32) {
                chain_currencies_by_chain_id.insert(Chain::Local as i32, chain_currency);
                chain_currencies_by_chain_id.insert(Chain::LocalAlt as i32, chain_currency);
            }
            chain_currencies_by_chain_id
        },
    );

    Ok(Json(
        games
            .iter()
            .map(|game| {
                let chain_currency = chain_currencies_by_chain_id.get(&game.chain_id).unwrap();

                GameResponse::new(game, *chain_currency)
            })
            .collect(),
    ))
}

#[derive(Debug, Deserialize)]
pub struct GetGameParams {
    pub player_address: Option<String>,
}

pub async fn get_game(
    State(app_state): State<AppState>,
    Path(id): Path<i64>,
    Query(GetGameParams { player_address }): Query<GetGameParams>,
) -> Result<Json<GameResponse>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let game = Repo::get_game(&mut conn, id).await;

    match game {
        Some(game) => {
            let chain_currency =
                Repo::get_chain_currency(&mut conn, game.get_chain_id()).await.unwrap();

            //separate state fetching from stateless computations - Why I love functional
            let game_response = GameResponse::new(&game, &chain_currency);

            if let Some(player_address) = player_address {
                let maybe_game_play =
                    Repo::get_game_play(&mut conn, game.id, &player_address).await;

                Ok(Json(
                    game_response
                        .clone()
                        .maybe_set_is_awaiting_my_play_proof(&game, &maybe_game_play)
                        .maybe_set_my_game_play_id(&maybe_game_play),
                ))
            } else {
                Ok(Json(game_response))
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Game not found".to_string())),
    }
}
