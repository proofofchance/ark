use std::sync::Arc;

// use guessbucket_db::DBPool;
use ark_db::DBPool;

// // Our shared state
#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DBPool>,
}

impl AppState {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self { db_pool }
    }
}
