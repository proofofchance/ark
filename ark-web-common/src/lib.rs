use std::sync::Arc;

use ark_db::DBPool;
use chaindexing::KeepNodeActiveRequest;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DBPool>,
    pub keep_chaindexing_node_active_request: KeepNodeActiveRequest,
}

impl AppState {
    pub fn new(
        db_pool: Arc<DBPool>,
        keep_chaindexing_node_active_request: &KeepNodeActiveRequest,
    ) -> Self {
        Self {
            db_pool,
            keep_chaindexing_node_active_request: keep_chaindexing_node_active_request.clone(),
        }
    }
}
