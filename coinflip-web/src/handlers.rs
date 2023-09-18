use http::StatusCode;

pub mod game_handler;

pub type Error = (StatusCode, String);
