use ark_db::schema;
use ark_db::DBConn;

use coinflip::UnsavedGameActivity;
use coinflip::{Game, GameActivity, GamePlay, GameStatus};

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
    pub page_size: Option<i64>,
    pub id_to_ignore: Option<i64>,
    pub status: Option<GameStatus>,
    pub reject_status: Option<GameStatus>,
    pub is_completed: Option<bool>,
    pub is_refunded: Option<bool>,
    pub chain_id_to_ignore: Option<i64>,
    pub offset: Option<u64>,
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

const MAX_GAMES_COUNT: i64 = 40;
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
            page_size,
            id_to_ignore,
            status: Some(GameStatus::AwaitingPlayers),
            ..
        } => {
            let page_size = page_size.unwrap_or(MAX_GAMES_COUNT);

            let mut query = coinflip_games
                .filter(expiry_timestamp.gt(now))
                .filter(completed_at.is_null())
                .order_by(chain_agnostic_index.desc())
                .limit(page_size)
                .into_boxed();

            query = if let Some(id_to_ignore) = id_to_ignore {
                query.filter(id.ne(id_to_ignore))
            } else {
                query
            };

            query.load(conn).await.unwrap()
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
                        .and(chain_id.eq(schema::coinflip_games::chain_id))
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(completed_at.is_null())
                .filter(expiry_timestamp.gt(now))
                .order_by(chain_agnostic_index.desc())
                .select(schema::coinflip_games::all_columns)
                // TODO: Post MVP pagination
                .limit(MAX_GAMES_COUNT)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: None,
            status: Some(GameStatus::Completed),
            ..
        } => {
            coinflip_games
                .filter(completed_at.is_not_null().or(expiry_timestamp.le(now)))
                .order_by(chain_agnostic_index.desc())
                // TODO: Post MVP pagination
                .limit(MAX_GAMES_COUNT)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            status: Some(GameStatus::Completed),
            ..
        } => {
            use ark_db::schema::coinflip_game_plays::dsl::*;

            coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(chain_id.eq(schema::coinflip_games::chain_id))
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .filter(completed_at.is_not_null().or(expiry_timestamp.le(now)))
                .filter(expiry_timestamp.gt(now))
                .order_by(chain_agnostic_index.desc())
                .select(schema::coinflip_games::all_columns)
                // TODO: Post MVP pagination
                .limit(MAX_GAMES_COUNT)
                .load(conn)
                .await
                .unwrap()
        }

        GetGamesParams {
            player_address: None,
            chain_id_to_ignore,
            offset,
            ..
        } => {
            let offset = offset.unwrap_or(0);

            let mut query = coinflip_games
                .order_by(chain_agnostic_index.desc())
                .limit(MAX_GAMES_COUNT)
                .offset(offset as i64)
                .into_boxed();

            query = if let Some(chain_id_to_ignore) = chain_id_to_ignore {
                query.filter(chain_id.ne(chain_id_to_ignore))
            } else {
                query
            };

            query.load(conn).await.unwrap()
        }

        GetGamesParams {
            player_address: Some(player_address_),
            chain_id_to_ignore,
            offset,
            ..
        } => {
            let offset = offset.unwrap_or(0);
            use ark_db::schema::coinflip_game_plays::dsl::*;

            let mut query = coinflip_games
                .inner_join(
                    coinflip_game_plays.on(game_id
                        .eq(schema::coinflip_games::id)
                        .and(chain_id.eq(schema::coinflip_games::chain_id))
                        .and(player_address.eq(player_address_.to_lowercase()))),
                )
                .order_by(chain_agnostic_index.desc())
                .select(schema::coinflip_games::all_columns)
                .limit(MAX_GAMES_COUNT)
                .offset(offset as i64)
                .into_boxed();

            query = if let Some(chain_id_to_ignore) = chain_id_to_ignore {
                query.filter(chain_id.eq(chain_id_to_ignore))
            } else {
                query
            };

            query.load(conn).await.unwrap()
        }
    }
}

pub async fn get_total_games_count<'a>(conn: &mut DBConn<'a>) -> u64 {
    use ark_db::schema::coinflip_games::dsl::*;

    coinflip_games.count().get_result::<i64>(conn).await.unwrap() as u64
}

pub async fn get_total_completed_games_count<'a>(conn: &mut DBConn<'a>) -> u64 {
    use ark_db::schema::coinflip_games::dsl::*;

    coinflip_games
        .filter(completed_at.is_not_null())
        .count()
        .get_result::<i64>(conn)
        .await
        .unwrap() as u64
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
    game_and_chain_ids: &Vec<(i64, i64)>,
) -> Vec<GamePlay> {
    use ark_db::schema::coinflip_game_plays::dsl::*;

    let mut query = coinflip_game_plays.into_boxed();

    for (game_id_, chain_id_) in game_and_chain_ids.iter() {
        query = query.or_filter(
            game_id
                .eq(game_id_)
                .and(chain_id.eq(chain_id_))
                .and(chance_and_salt.is_not_null()),
        )
    }

    query.load(conn).await.unwrap()
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
