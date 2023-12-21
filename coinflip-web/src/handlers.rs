use std::sync::Arc;

use http::StatusCode;

pub mod game_activity_handler;
pub mod game_handler;
pub mod game_play_handler;
use serde::Serialize;

pub type Error = (StatusCode, String);

#[derive(Debug, Serialize)]
pub struct GenericMessage {
    message: String,
}

impl GenericMessage {
    pub fn new(message: &str) -> Self {
        GenericMessage {
            message: message.to_string(),
        }
    }
}

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
