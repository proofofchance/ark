use std::{collections::HashMap, hash::Hash};

use chrono::Utc;

pub struct RecentCache<K: Hash + Eq, V> {
    data: HashMap<K, (V, u64)>,
    stale_after: u64,
}

impl<K: Hash + Eq, V> RecentCache<K, V> {
    pub fn new(stale_after: u64) -> Self {
        Self {
            data: HashMap::new(),
            stale_after,
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key).and_then(|(value, inserted_at)| {
            if now() - inserted_at < self.stale_after {
                Some(value)
            } else {
                None
            }
        })
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, (value, now()));
    }
}

fn now() -> u64 {
    Utc::now().timestamp() as u64
}
