use ark_repo::{GetGamesParams, Repo};
use ark_web_app::AppState;
use axum::{
    extract::{Query, State},
    Json,
};

use coinflip::{Game, GameStatus};
use serde::{Deserialize, Serialize};

use crate::handlers;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameResponse {
    id: u64,
    expiry_timestamp: u64,
    creator_address: String,
    block_number: u64,
    status: GameStatus,
    wager: String,
    wager_usd: f64,
    total_possible_win_usd: f64,
    players_left: u32,
    total_players_required: u32,
    view_count: u64,
}

impl From<&Game> for GameResponse {
    fn from(game: &Game) -> Self {
        let total_players_required = game.max_play_count as u32;

        // TODO: Use https://github.com/rust-num/num-bigint
        // Fetch using: https://ethereum.stackexchange.com/questions/38309/what-are-the-popular-api-to-get-current-exchange-rates-for-ethereum-to-usd
        // Example: https://min-api.cryptocompare.com/data/price?fsym=MATIC&tsyms=USD
        let wager_usd = 0.1;

        GameResponse {
            id: game.id as u64,
            expiry_timestamp: game.expiry_timestamp as u64,
            creator_address: game.creator_address.clone(),
            block_number: game.block_number as u64,
            status: game.get_status(),
            wager: game.wager.clone(),
            wager_usd,
            total_possible_win_usd: total_players_required as f64 * wager_usd,
            players_left: game.get_players_left(),
            total_players_required,
            // TODO
            view_count: 0,
        }
    }
}

pub async fn get_games(
    State(app_state): State<AppState>,
    query_params: Query<GetGamesParams>,
) -> Result<Json<Vec<GameResponse>>, handlers::Error> {
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let games = Repo::get_all_games(&mut conn, &query_params).await;

    Ok(Json(games.iter().map(|game| game.into()).collect()))
}
