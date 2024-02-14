use std::sync::Arc;

use ark_db::DBPool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DBPool>,
}

impl AppState {
    pub fn new(db_pool: Arc<DBPool>) -> Self {
        Self { db_pool }
    }
}
