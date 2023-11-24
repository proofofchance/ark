use ark_db::DBConn;

use coinflip::{
    chains::{ChainCurrency, UnsavedChainCurrency},
    Game, GameField, GamePlay, GameStatus,
};
use diesel::{
    upsert::excluded, BoolExpressionMethods, ExpressionMethods, OptionalExtension, QueryDsl,
};
use diesel_async::RunQueryDsl;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize)]
pub struct GetGamesParams {
    pub creator_address: Option<String>,
    pub order_by_field: Option<(GameField, Order)>,
    pub status: Option<GameStatus>,
}

pub struct Repo;

impl Repo {
    pub async fn get_game<'a>(conn: &mut DBConn<'a>, id_: i64) -> Option<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        coinflip_games.filter(id.eq(id_)).first(conn).await.optional().unwrap()
    }

    pub async fn get_all_games<'a>(conn: &mut DBConn<'a>, params: &GetGamesParams) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        let now = chrono::offset::Utc::now().timestamp();

        match params {
            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                status: Some(GameStatus::Ongoing),
            } => coinflip_games
                .filter(is_completed.eq(false))
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                status: Some(GameStatus::Completed),
            } => {
                coinflip_games
                    .filter(is_completed.eq(true).or(expiry_timestamp.le(now)))
                    .order_by(block_number.desc())
                    // TODO: Post MVP pagination
                    .limit(100)
                    .load(conn)
                    .await
                    .unwrap()
            }

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                status: Some(GameStatus::Ongoing),
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .filter(is_completed.eq(false))
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                status: Some(GameStatus::Completed),
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .filter(is_completed.eq(true).or(expiry_timestamp.le(now)))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                status: None,
            } => coinflip_games
                .order_by(block_number.desc())
                .order_by(is_completed.asc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                status: None,
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .order_by(block_number.desc())
                .order_by(is_completed.asc())
                .load(conn)
                .await
                .unwrap(),

            _ => unimplemented!("GetGameParams combination not yet implemented!"),
        }
    }

    pub async fn get_game_plays<'a>(conn: &mut DBConn<'a>, game_id_: i64) -> Vec<GamePlay> {
        use ark_db::schema::coinflip_game_plays::dsl::*;

        coinflip_game_plays.filter(game_id.eq(game_id_)).load(conn).await.unwrap()
    }

    pub async fn create_or_update_chain_currencies<'a>(
        conn: &mut DBConn<'a>,
        chain_currencies: &Vec<UnsavedChainCurrency>,
    ) {
        use ark_db::schema::coinflip_chain_currencies::dsl::*;

        diesel::insert_into(coinflip_chain_currencies)
            .values(chain_currencies)
            .on_conflict((chain_id, currency_symbol))
            .do_update()
            .set(unit_usd_price.eq(excluded(unit_usd_price)))
            .execute(conn)
            .await
            .unwrap();
    }

    pub async fn get_chain_currencies<'a>(
        conn: &mut DBConn<'a>,
        chain_ids: &Vec<i32>,
    ) -> Vec<ChainCurrency> {
        use ark_db::schema::coinflip_chain_currencies::dsl::*;

        coinflip_chain_currencies
            .filter(chain_id.eq_any(chain_ids))
            .load(conn)
            .await
            .unwrap()
    }

    pub async fn get_chain_currency<'a>(
        conn: &mut DBConn<'a>,
        chain_id_: i32,
    ) -> Option<ChainCurrency> {
        use ark_db::schema::coinflip_chain_currencies::dsl::*;

        coinflip_chain_currencies
            .filter(chain_id.eq(chain_id_))
            .first(conn)
            .await
            .optional()
            .unwrap()
    }
}
