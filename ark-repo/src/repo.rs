use ark_db::DBConn;

use coinflip::{Game, GameField, GameStatus};
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
    pub status: Option<GameStatus>,
    pub order_by_field: Option<(GameField, Order)>,
}

pub struct Repo;

impl Repo {
    pub async fn get_all_games<'a>(conn: &mut DBConn<'a>, params: &GetGamesParams) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        match params {
            GetGamesParams {
                creator_address: None,
                status: _status,
                order_by_field: None,
            } => coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap(),
            GetGamesParams {
                creator_address: None,
                status: _status,
                order_by_field: Some((GameField::BlockNumber, Order::Asc)),
            } => coinflip_games.order_by(block_number.asc()).load(conn).await.unwrap(),
            GetGamesParams {
                creator_address: None,
                status: _status,
                order_by_field: Some((GameField::BlockNumber, Order::Desc)),
            } => coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap(),

            GetGamesParams {
                creator_address: Some(creator_address_),
                status: _status,
                order_by_field: None,
            } => coinflip_games
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .order_by(block_number.desc())
                .load(conn)
                .await
                .unwrap(),
            GetGamesParams {
                creator_address: Some(creator_address_),
                status: _status,
                order_by_field: Some((GameField::BlockNumber, Order::Asc)),
            } => coinflip_games
                .order_by(block_number.asc())
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .load(conn)
                .await
                .unwrap(),
            GetGamesParams {
                creator_address: Some(creator_address_),
                status: _status,
                order_by_field: Some((GameField::BlockNumber, Order::Desc)),
            } => coinflip_games
                .order_by(block_number.desc())
                .filter(creator_address.eq(creator_address_.to_lowercase()))
                .load(conn)
                .await
                .unwrap(),

            _ => unimplemented!("GetGameParams combination not yet implemented!"),
        }
    }
}
