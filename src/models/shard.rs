use dashmap::DashMap;
use std::sync::Arc;

use super::KvPair;

#[derive(Clone, Debug)]
pub struct ShardEndpoint {
    pub id: String,
    pub address: String,
}

pub struct Shard {
    store: Arc<DashMap<String, KvPair>>,
}

impl Shard {
    pub fn new() -> Self {
        Shard {
            store: Arc::new(DashMap::new()),
        }
    }

    pub fn set(&self, key: String, value: String) {
        self.store.insert(key.clone(), KvPair::new(key.clone(), value));
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).map(|kv_pair| kv_pair.value.clone())
    }
}
