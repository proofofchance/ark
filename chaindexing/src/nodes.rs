use diesel::{prelude::Insertable, Queryable};
use serde::Deserialize;

use crate::diesels::schema::chaindexing_nodes;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Insertable, Queryable)]
#[diesel(table_name = chaindexing_nodes)]
pub struct Node {
    pub id: i32,
    last_active_at: i64,
    inserted_at: i64,
}

impl Node {
    pub const ELECTION_RATE_SECS: u64 = 60;

    pub fn get_min_active_at() -> i64 {
        let now = chrono::Utc::now().timestamp();

        // Not active if not kept active at least 2 elections away
        now - (Node::ELECTION_RATE_SECS * 2) as i64
    }
}

pub fn elect_leader<'a>(nodes: &'a Vec<Node>) -> &'a Node {
    let mut nodes_iter = nodes.iter();
    let mut leader: Option<&Node> = nodes_iter.next();

    while let Some(node) = nodes_iter.next() {
        if node.inserted_at > leader.unwrap().inserted_at {
            leader = Some(node);
        }
    }

    leader.unwrap()
}
