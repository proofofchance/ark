use ark_db::schema;
use ark_db::DBConn;

use coinflip::UnsavedGameActivity;
use coinflip::{Game, GameActivity, GameField, GamePlay, GameStatus};

use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl};
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
    pub reject_status: Option<GameStatus>,
    pub is_completed: Option<bool>,
    pub is_refunded: Option<bool>,
    pub chain_id_to_ignore: Option<i64>,
}

impl GetGamesParams {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn expired(mut self) -> Self {
        self.status = Some(GameStatus::Expired);
        self
    }
    pub fn not_expired(mut self) -> Self {
        self.reject_status = Some(GameStatus::Expired);
        self
    }
    pub fn not_refunded(mut self) -> Self {
        self.is_refunded = Some(false);
        self
    }
    pub fn only_incomplete(mut self) -> Self {
        self.is_completed = Some(false);
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
            is_refunded: Some(false),
            ..
        } => coinflip_games
            .filter(expiry_timestamp.lt(now))
            .filter(refunded_at.is_null())
            .filter(completed_at.is_null())
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            reject_status: Some(GameStatus::Expired),
            is_completed: Some(false),
            ..
        } => coinflip_games
            .filter(expiry_timestamp.gt(now))
            .filter(completed_at.is_null())
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            player_address: None,
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            status: Some(GameStatus::AwaitingPlayers),
            ..
        } => coinflip_games
            .filter(completed_at.is_null())
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
            status: Some(GameStatus::AwaitingPlayers),
            ..
        } => coinflip_games
            .filter(id.ne(id_to_ignore))
            .filter(expiry_timestamp.gt(now))
            .filter(completed_at.is_null())
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
            status: Some(GameStatus::Completed),
            ..
        } => {
            coinflip_games
                .filter(completed_at.is_not_null().or(expiry_timestamp.le(now)))
                .order_by(block_number.desc())
                // TODO: Post MVP pagination
                .limit(200)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            status: Some(GameStatus::AwaitingPlayers),
            ..
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(completed_at.is_null())
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .select(schema::coinflip_games::all_columns)
                // TODO: Post MVP pagination
                .limit(200)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            order_by_field: None,
            page_size: None,
            id_to_ignore: None,
            status: Some(GameStatus::Completed),
            ..
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(completed_at.is_not_null().or(expiry_timestamp.le(now)))
                .filter(expiry_timestamp.gt(now))
                .order_by(block_number.desc())
                .select(schema::coinflip_games::all_columns)
                // TODO: Post MVP pagination
                .limit(200)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: None,
            chain_id_to_ignore: Some(chain_id_to_ignore),
            ..
        } => coinflip_games
            .order_by(block_number.desc())
            // TODO: Post MVP pagination
            .limit(200)
            .filter(chain_id.ne(chain_id_to_ignore))
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            player_address: None,
            chain_id_to_ignore: None,
            ..
        } => coinflip_games
            .order_by(block_number.desc())
            // TODO: Post MVP pagination
            .limit(200)
            .load(conn)
            .await
            .unwrap(),

        GetGamesParams {
            player_address: Some(player_address_),
            chain_id_to_ignore: Some(chain_id_to_ignore),
            ..
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .order_by(block_number.desc())
                .filter(chain_id.eq(chain_id_to_ignore))
                .select(schema::coinflip_games::all_columns)
                // TODO: Post MVP pagination
                .limit(200)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            chain_id_to_ignore: None,
            ..
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
                // TODO: Post MVP pagination
                .limit(200)
                .load(conn)
                .await
                .unwrap()
        }
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

pub async fn get_game_play_by_id<'a>(
    conn: &mut DBConn<'a>,
    game_id_: i64,
    chain_id_: i64,
    game_play_id: i32,
) -> Option<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    coinflip_game_plays
        .filter(id.eq(game_play_id))
        .filter(game_id.eq(game_id_))
        .filter(chain_id.eq(chain_id_))
        .first(conn)
        .await
        .optional()
        .unwrap()
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

pub async fn create_game_activity<'a>(conn: &mut DBConn<'a>, game_activity: &UnsavedGameActivity) {
    use ark_db::schema::coinflip_game_activities::dsl::*;

    diesel::insert_into(coinflip_game_activities)
        .values(game_activity)
        .execute(conn)
        .await
        .unwrap();
}

pub struct GetGameActivityParams {
    pub game_id: i64,
    pub chain_id: i64,
    pub trigger_public_address: String,
    pub kind: String,
}

pub async fn get_game_activity<'a>(
    conn: &mut DBConn<'a>,
    params: &GetGameActivityParams,
) -> Option<GameActivity> {
    use ark_db::schema::coinflip_game_activities::dsl::*;

    coinflip_game_activities
        .filter(game_id.eq(params.game_id))
        .filter(chain_id.eq(params.chain_id))
        .filter(trigger_public_address.eq(&params.trigger_public_address))
        .filter(kind.eq(&params.kind))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

pub async fn update_game_activity_transaction_hash<'a>(
    conn: &mut DBConn<'a>,
    game_activity: &GameActivity,
    transaction_hash_: &str,
) {
    use ark_db::schema::coinflip_game_activities::dsl::*;

    diesel::update(coinflip_game_activities)
        .filter(id.eq(game_activity.id))
        .set(transaction_hash.eq(transaction_hash_))
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
        .order_by(id.desc())
        .load(conn)
        .await
        .unwrap()
}
