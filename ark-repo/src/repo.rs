use ark_db::DBConn;

use coinflip::{Game, GameField, GamePlay, GameStatus};
use diesel::{ExpressionMethods, QueryDsl};
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
    pub async fn get_all_games<'a>(conn: &mut DBConn<'a>, params: &GetGamesParams) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        match params {
            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                status: Some(GameStatus::Available),
            } => coinflip_games
                .filter(is_completed.eq(false))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                status: Some(GameStatus::Completed),
            } => coinflip_games
                .filter(is_completed.eq(true))
                .order_by(block_number.desc())
                // TODO: Post MVP pagination
                .limit(100)
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                status: Some(GameStatus::Available),
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .filter(is_completed.eq(false))
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
                .filter(is_completed.eq(true))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),

            GetGamesParams {
                creator_address: None,
                order_by_field: None,
                ..
            } => coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                order_by_field: None,
                ..
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
}
