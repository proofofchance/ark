use ark_db::DBConn;

use coinflip::{
    chains::{ChainCurrency, UnsavedChainCurrency},
    Game, GameActivity, GameField, GamePlay, GamePlayProof, GameStatus,
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
    pub page_size: Option<i64>,
    pub id_to_ignore: Option<i64>,
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
                page_size: None,
                id_to_ignore: None,
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
                page_size: Some(page_size),
                id_to_ignore: Some(id_to_ignore),
                status: Some(GameStatus::Ongoing),
            } => coinflip_games
                .filter(id.ne(id_to_ignore))
                .filter(expiry_timestamp.gt(now))
                .filter(is_completed.eq(false))
                .order_by(block_number.desc())
                .limit(*page_size)
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                page_size: None,
                id_to_ignore: None,
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
                page_size: None,
                id_to_ignore: None,
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
                page_size: None,
                id_to_ignore: None,
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
                page_size: None,
                id_to_ignore: None,
            } => coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                page_size: None,
                status: None,
                id_to_ignore: None,
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .order_by(block_number.desc())
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

    pub async fn get_ongoing_game_ids<'a>(
        conn: &mut DBConn<'a>,
        player_address_: &str,
    ) -> Vec<i64> {
        use ark_db::schema::coinflip_game_plays::dsl::*;

        coinflip_game_plays
            .select(game_id)
            .filter(player_address.eq(player_address_))
            .load(conn)
            .await
            .unwrap()
    }

    pub async fn get_game_play<'a>(
        conn: &mut DBConn<'a>,
        game_id_: i64,
        player_address_: &str,
    ) -> Option<GamePlay> {
        use ark_db::schema::coinflip_game_plays::dsl::*;

        coinflip_game_plays
            .filter(game_id.eq(game_id_))
            .filter(player_address.eq(player_address_.to_lowercase()))
            .first(conn)
            .await
            .optional()
            .unwrap()
    }

    pub async fn get_game_play_proof<'a>(
        conn: &mut DBConn<'a>,
        game_id_: i64,
        player_address_: &str,
    ) -> Option<GamePlayProof> {
        use ark_db::schema::coinflip_game_play_proofs::dsl::*;

        coinflip_game_play_proofs
            .filter(game_id.eq(game_id_))
            .filter(player_address.eq(player_address_.to_lowercase()))
            .first(conn)
            .await
            .optional()
            .unwrap()
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

    pub async fn get_game_activities<'a>(
        conn: &mut DBConn<'a>,
        game_ids: &Vec<i64>,
    ) -> Vec<GameActivity> {
        use ark_db::schema::coinflip_game_activities::dsl::*;

        coinflip_game_activities
            .filter(game_id.eq_any(game_ids))
            .load(conn)
            .await
            .unwrap()
    }
}
