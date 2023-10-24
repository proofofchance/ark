use ark_db::DBConn;

use coinflip::Game;
use diesel_async::RunQueryDsl;

#[derive(Clone)]
pub struct Repo;

impl Repo {
    pub async fn get_all_games<'a>(conn: &mut DBConn<'a>) -> Vec<Game> {
        use ark_db::schema::coinflip_games::dsl::*;

        coinflip_games.load(conn).await.unwrap()
    }
}
