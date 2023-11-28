use std::sync::Arc;

use http::StatusCode;

pub mod game_activity_handler;
pub mod game_handler;

pub type Error = (StatusCode, String);

use ark_db::{DBConn, DBPool};

pub async fn new_conn<'a>(pool: Arc<DBPool>) -> Result<DBConn<'a>, Error> {
    Ok(pool.get_owned().await.map_err(internal_error)?)
}

pub fn internal_error<E>(err: E) -> Error
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
