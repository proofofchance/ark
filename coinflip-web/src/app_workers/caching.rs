use std::{collections::HashMap, hash::Hash};

use chrono::Utc;

pub struct RecentCache<K: Hash + Eq + Clone, V> {
    data: HashMap<K, (V, u64)>,
    stale_after: u64,
}

impl<K: Hash + Eq + Clone, V> RecentCache<K, V> {
    pub fn new(stale_after: u64) -> Self {
        Self {
            data: HashMap::new(),
            stale_after,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.data.insert(key, (value, now()));
    }
    pub fn invalidate_all(&mut self) {
        let keys: Vec<_> = self.data.keys().cloned().collect();
        for key in keys.iter() {
            self.invalidate(key);
        }
    }
    pub fn invalidate(&mut self, key: &K) {
        if let Some((_value, mut _inserted_at)) = self.data.get_mut(key) {
            _inserted_at = 0;
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
}

fn now() -> u64 {
    Utc::now().timestamp() as u64
}
