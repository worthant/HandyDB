use std::sync::Arc;

use crate::models::KvPair;
use dashmap::DashMap;

pub struct KvStore {
    store: Arc<DashMap<String, KvPair>>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: Arc::new(DashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        self.store
            .insert(key.clone(), KvPair::new(key.clone(), value));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|kv_pair| kv_pair.value.clone())
    }
}