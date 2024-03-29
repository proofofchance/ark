use std::collections::HashMap;

use ark_utils::floats;
use ark_web_common::AppState;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use coinflip_repo::GetGamesParams;

use ark_web3::chains::{ChainCurrency, ChainId};
use coinflip::{Game, GamePlay, GameStatus, PlayerAddress};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::handlers;

#[derive(Debug, Serialize)]
pub struct PaginatedGames {
    games: Vec<GameResponse>,
    total_completed_games_count: u64,
    total_games_count: u64,
    total_paid_out_amount: f64,
}
pub async fn get_games(
    State(app_state): State<AppState>,
    query_params: Query<GetGamesParams>,
) -> Result<Json<PaginatedGames>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = coinflip_repo::get_games(&mut conn, &query_params).await;

    let chain_ids: Vec<_> = games.iter().map(|game| game.get_chain_id()).collect();

    let chain_currencies = ark_repo::get_chain_currencies(&mut conn, &chain_ids).await;
    let chain_currencies_by_chain_id = chain_currencies.iter().fold(
        HashMap::new(),
        |mut chain_currencies_by_chain_id, chain_currency| {
            chain_currencies_by_chain_id.insert(chain_currency.chain_id, chain_currency);

            if chain_currency.chain_id == (ChainId::Ethereum as i64) {
                chain_currencies_by_chain_id.insert(ChainId::Local as i64, chain_currency);
                chain_currencies_by_chain_id.insert(ChainId::LocalAlt as i64, chain_currency);
            }
            chain_currencies_by_chain_id
        },
    );

    let games = games
        .iter()
        .map(|game| {
            let chain_currency = chain_currencies_by_chain_id.get(&game.chain_id).unwrap();

            GameResponse::new(game, *chain_currency)
        })
        .collect();
    let total_completed_games_count =
        coinflip_repo::get_total_completed_games_count(&mut conn).await;
    let total_games_count = coinflip_repo::get_total_games_count(&mut conn).await;
    let total_paid_out_report = ark_repo::get_last_total_paid_out_report(&mut conn).await;
    let total_paid_out_amount = total_paid_out_report.map(|r| r.get_amount()).unwrap_or_default();

    Ok(Json(PaginatedGames {
        games,
        total_completed_games_count,
        total_games_count,
        total_paid_out_amount: floats::to_2dp(total_paid_out_amount),
    }))
}

#[derive(Debug, Deserialize)]
pub struct GetGameParams {
    pub player_address: Option<String>,
}

pub async fn get_game(
    State(app_state): State<AppState>,
    Path((id, chain_id)): Path<(u64, u64)>,
    Query(GetGameParams { player_address }): Query<GetGameParams>,
) -> Result<Json<GameResponse>, handlers::Error> {
    let id = id as i64;
    let chain_id = chain_id as i64;

    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let game = coinflip_repo::get_game(&mut conn, id, chain_id).await;
    let chain_currency = ark_repo::get_chain_currency(&mut conn, chain_id).await.unwrap();

    match game {
        Some(game) => {
            let game_response = GameResponse::new(&game, &chain_currency);
            let game_plays = coinflip_repo::get_game_plays(&mut conn, game.id, chain_id).await;

            if let Some(player_address) = player_address {
                let maybe_game_play = game_plays
                    .iter()
                    .find(|gp| PlayerAddress::do_both_match(&gp.player_address, &player_address))
                    .cloned();

                let game_response = game_response
                    .maybe_set_is_awaiting_my_chance_reveal(&game, &maybe_game_play)
                    .maybe_set_my_game_play_id(&maybe_game_play);

                Ok(Json(game_response.maybe_include_completed_game_data(
                    &game_plays,
                    &chain_currency,
                )))
            } else {
                Ok(Json(game_response.maybe_include_completed_game_data(
                    &game_plays,
                    &chain_currency,
                )))
            }
        }
        None => Err((StatusCode::NOT_FOUND, "Game not found".to_string())),
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevealedProofOfChance {
    pub player_address: String,
    pub chance_and_salt: String,
    pub proof_of_chance: String,
}

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
    unavailable_coin_side: Option<i32>,
    is_awaiting_my_chance_reveal: Option<bool>,
    my_game_play_id: Option<i32>,
    revealed_proof_of_chances: Option<Vec<RevealedProofOfChance>>,
    outcome: Option<i32>,
    completed_at: Option<i64>,
    game_plays: Option<Vec<GamePlay>>,
    amount_for_each_winner: Option<f64>,
    amount_for_each_winner_usd: Option<f64>,
    amount_shared_with_winners: Option<f64>,
    amount_shared_with_winners_usd: Option<f64>,
    refunded_amount_per_player: Option<f64>,
    refunded_at: Option<i64>,
}

impl GameResponse {
    fn new(game: &Game, chain_currency: &ChainCurrency) -> Self {
        let total_players_required = game.number_of_players as u32;

        let wager = game.get_wager_ether();
        let wager_usd = chain_currency.convert_to_usd(wager);
        let wager_usd = floats::to_2dp(wager_usd);

        GameResponse {
            id: game.id as u64,
            chain_id: game.chain_id as u32,
            expiry_timestamp: game.expiry_timestamp as u64,
            creator_address: game.creator_address.clone(),
            block_number: game.block_number as u64,
            outcome: game.outcome,
            completed_at: game.completed_at,
            status: game.get_status(),
            wager,
            wager_usd,
            // TODO: Should be calculated from the number of heads and tails so far (whichever has most)
            // If no play yet, then it is total players required * wager usd
            max_possible_win_usd: Game::deduct_service_charge(
                total_players_required as f64 * wager_usd,
            ),
            players_left: game.get_players_left(),
            total_players_required,
            is_awaiting_my_chance_reveal: None, // view_count: 0,
            unavailable_coin_side: game.unavailable_coin_side,
            my_game_play_id: None,
            revealed_proof_of_chances: None,
            game_plays: None,
            amount_for_each_winner: game.get_amount_for_each_winner_ether(),
            amount_for_each_winner_usd: None,
            amount_shared_with_winners: None,
            amount_shared_with_winners_usd: None,
            refunded_at: game.refunded_at,
            refunded_amount_per_player: game.get_refunded_amount_per_player_ether(),
        }
    }

    fn maybe_set_is_awaiting_my_chance_reveal(
        mut self,
        game: &Game,
        maybe_game_play: &Option<GamePlay>,
    ) -> Self {
        self.is_awaiting_my_chance_reveal = if !game.is_awaiting_revealed_chances() {
            None
        } else {
            maybe_game_play.as_ref().map(|game_play| game_play.chance_and_salt.is_none())
        };

        self
    }

    fn maybe_set_my_game_play_id(mut self, maybe_game_play: &Option<GamePlay>) -> Self {
        if let Some(GamePlay { id, .. }) = maybe_game_play {
            self.my_game_play_id = Some(*id);
        }

        self
    }

    fn maybe_include_completed_game_data(
        self,
        game_plays: &Vec<GamePlay>,
        chain_currency: &ChainCurrency,
    ) -> Self {
        if self.completed_at.is_some() {
            self.include_game_plays(game_plays)
                .include_revealed_proof_of_chances(game_plays)
                .include_amount_for_each_winner_usd(chain_currency)
                .include_amounts_shared_with_winners(game_plays, chain_currency)
        } else {
            self.include_game_plays(game_plays)
        }
    }
    fn include_game_plays(mut self, game_plays: &Vec<GamePlay>) -> Self {
        self.game_plays = Some(game_plays.clone());

        self
    }
    fn include_revealed_proof_of_chances(mut self, game_plays: &Vec<GamePlay>) -> Self {
        self.revealed_proof_of_chances = Some(
            game_plays
                .into_iter()
                .map(|gp| RevealedProofOfChance {
                    player_address: gp.player_address.to_owned(),
                    chance_and_salt: gp.chance_and_salt.clone().unwrap(),
                    proof_of_chance: gp.proof_of_chance.to_owned(),
                })
                .collect(),
        );

        self
    }
    fn include_amount_for_each_winner_usd(mut self, chain_currency: &ChainCurrency) -> Self {
        self.amount_for_each_winner_usd =
            self.amount_for_each_winner.map(|amt| chain_currency.convert_to_usd(amt));
        self
    }
    fn include_amounts_shared_with_winners(
        mut self,
        game_plays: &Vec<GamePlay>,
        chain_currency: &ChainCurrency,
    ) -> Self {
        let winners =
            self.outcome.map(|outcome| GamePlay::filter_by_coin_side(game_plays, outcome));
        let winners_count = winners.map(|winners| winners.len());

        if let Some((winners_count, amount_for_each_winner)) =
            winners_count.zip(self.amount_for_each_winner)
        {
            let amount_shared_with_winners = amount_for_each_winner * (winners_count as f64);
            self.amount_shared_with_winners = Some(amount_shared_with_winners);
            self.amount_shared_with_winners_usd = Some(floats::to_2dp(
                chain_currency.convert_to_usd(amount_shared_with_winners),
            ));
        }

        self
    }
}
