use ark_db::schema;
use ark_db::DBConn;

use coinflip::UnsavedGameActivity;
use coinflip::{
    chains::{ChainCurrency, UnsavedChainCurrency},
    Game, GameActivity, GameField, GamePlay, GameStatus,
};

use diesel::{
    upsert::excluded, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension,
    QueryDsl,
};
use diesel_async::RunQueryDsl;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize, Default)]
pub struct GetGamesParams {
    pub player_address: Option<String>,
    pub order_by_field: Option<(GameField, Order)>,
    pub page_size: Option<i64>,
    pub id_to_ignore: Option<i64>,
    pub status: Option<GameStatus>,
    pub are_chances_revealed: Option<bool>,
}

impl GetGamesParams {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn reject_game_status(mut self, game_status: GameStatus) -> Self {
        self.status = Some(game_status);
        self
    }

    pub fn filter_chances_not_revealed(mut self) -> Self {
        self.are_chances_revealed = Some(false);
        self
    }
}

pub async fn get_game<'a>(conn: &mut DBConn<'a>, id_: i64, chain_id_: i64) -> Option<Game> {
    use ark_db::schema::coinflip_games::dsl::*;

    coinflip_games
        .filter(id.eq(id_))
        .filter(chain_id.eq(chain_id_))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

pub async fn get_games<'a>(conn: &mut DBConn<'a>, params: &GetGamesParams) -> Vec<Game> {
    use ark_db::schema::coinflip_games::dsl::*;

    let now = chrono::offset::Utc::now().timestamp();

    match params {
        GetGamesParams {
            status: Some(GameStatus::Expired),
            are_chances_revealed: Some(false),
            ..
        } => coinflip_games
            .filter(expiry_timestamp.gt(now))
            .filter(chances_revealed_at.is_null())
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            player_address: None,
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            are_chances_revealed: None,
            status: Some(GameStatus::Ongoing),
        } => coinflip_games
            .filter(is_completed.eq(false))
            .filter(expiry_timestamp.gt(now))
            .order_by(block_number.desc())
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            player_address: None,
            order_by_field: None,
            page_size: Some(page_size),
            id_to_ignore: Some(id_to_ignore),
            are_chances_revealed: None,
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
            player_address: None,
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            are_chances_revealed: None,
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
            player_address: Some(player_address_),
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            are_chances_revealed: None,
            status: Some(GameStatus::Ongoing),
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(is_completed.eq(false))
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .select(schema::coinflip_games::all_columns)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            are_chances_revealed: None,
            status: Some(GameStatus::Completed),
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(is_completed.eq(true).or(expiry_timestamp.le(now)))
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .select(schema::coinflip_games::all_columns)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: None,
            order_by_field: None,
            status: None,
            page_size: None,
            id_to_ignore: None,
            are_chances_revealed: None,
        } => coinflip_games.order_by(block_number.desc()).load(conn).await.unwrap(),

        GetGamesParams {
            player_address: Some(player_address_),
            order_by_field: None,
            page_size: None,
            status: None,
            id_to_ignore: None,
            are_chances_revealed: None,
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .order_by(block_number.desc())
                .select(schema::coinflip_games::all_columns)
                .load(conn)
                .await
                .unwrap()
        }

        _ => unimplemented!("GetGameParams combination not yet implemented!"),
    }
}

pub async fn get_game_plays<'a>(
    conn: &mut DBConn<'a>,
    game_id_: i64,
    chain_id_: i64,
) -> Vec<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    coinflip_game_plays
        .filter(game_id.eq(game_id_))
        .filter(chain_id.eq(chain_id_))
        .load(conn)
        .await
        .unwrap()
}

pub async fn get_all_game_plays_with_proofs<'a>(
    conn: &mut DBConn<'a>,
    game_ids: &Vec<i64>,
    chain_ids: &Vec<i64>,
) -> Vec<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    coinflip_game_plays
        .filter(game_id.eq_any(game_ids))
        .filter(chain_id.eq_any(chain_ids))
        .filter(chance_and_salt.is_not_null())
        .load(conn)
        .await
        .unwrap()
}

pub async fn get_game_plays_for_player<'a>(
    conn: &mut DBConn<'a>,
    player_address_: &str,
) -> Vec<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    coinflip_game_plays
        .filter(player_address.eq(player_address_.to_lowercase()))
        .load(conn)
        .await
        .unwrap()
}

pub async fn get_game_play<'a>(
    conn: &mut DBConn<'a>,
    game_id_: i64,
    chain_id_: i64,
    player_address_: &str,
) -> Option<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    coinflip_game_plays
        .filter(game_id.eq(game_id_))
        .filter(chain_id.eq(chain_id_))
        .filter(player_address.eq(player_address_.to_lowercase()))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

pub async fn record_chances_revealed<'a>(conn: &mut DBConn<'a>, id_: i64, chain_id_: i64) {
    use ark_db::schema::coinflip_games::dsl::*;

    let now = chrono::offset::Utc::now().timestamp();

    diesel::update(coinflip_games)
        .filter(id.eq(id_))
        .filter(chain_id.eq(chain_id_))
        .set(chances_revealed_at.eq(now))
        .execute(conn)
        .await
        .unwrap();
}

pub async fn update_game_play_chance_and_salt<'a>(
    conn: &mut DBConn<'a>,
    game_play: &GamePlay,
    chance_and_salt_: String,
) {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    diesel::update(coinflip_game_plays)
        .filter(id.eq(game_play.id))
        .filter(game_id.eq(game_play.game_id))
        .filter(chain_id.eq(game_play.chain_id))
        .set(chance_and_salt.eq(chance_and_salt_))
        .execute(conn)
        .await
        .unwrap();
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
    chain_ids: &Vec<i64>,
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
    chain_id_: i64,
) -> Option<ChainCurrency> {
    use ark_db::schema::coinflip_chain_currencies::dsl::*;

    coinflip_chain_currencies
        .filter(chain_id.eq(chain_id_))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

pub async fn create_game_activity<'a>(conn: &mut DBConn<'a>, game_activity: &UnsavedGameActivity) {
    use ark_db::schema::coinflip_game_activities::dsl::*;

    diesel::insert_into(coinflip_game_activities)
        .values(game_activity)
        .execute(conn)
        .await
        .unwrap();
}

pub async fn get_game_activities<'a>(
    conn: &mut DBConn<'a>,
    game_ids: &Vec<i64>,
    chain_ids: &Vec<i64>,
) -> Vec<GameActivity> {
    use ark_db::schema::coinflip_game_activities::dsl::*;

    coinflip_game_activities
        .filter(game_id.eq_any(game_ids))
        .filter(chain_id.eq_any(chain_ids))
        .load(conn)
        .await
        .unwrap()
}
