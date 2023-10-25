use ark_db::DBConn;

use coinflip::{Game, GameField};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;

#[derive(Clone)]
pub struct Repo;

pub enum Order {
    Asc,
    Desc,
}

impl Repo {
    pub async fn get_all_games<'a>(
        conn: &mut DBConn<'a>,
        field_order: (GameField, Order),
    ) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        match field_order {
            (GameField::BlockNumber, Order::Asc) => {
                coinflip_games.order_by(block_number.asc()).load(conn).await.unwrap()
            }
            (GameField::BlockNumber, Order::Desc) => {
                coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap()
            }
            (GameField::ExpiryTimestamp, Order::Asc) => {
                coinflip_games.order_by(block_number.asc()).load(conn).await.unwrap()
            }
            (GameField::ExpiryTimestamp, Order::Desc) => {
                coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap()
            }
            _ => unreachable!(),
        }
    }

    pub async fn get_creator_games<'a>(conn: &mut DBConn<'a>, creator_address_: &str) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        coinflip_games
            .order_by(block_number.asc())
            .filter(creator_address.eq(creator_address_.to_lowercase()))
            .load(conn)
            .await
            .unwrap()
    }
}
