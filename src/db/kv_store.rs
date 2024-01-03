
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::models::KvPair;

pub struct KvStore {
    store: Arc<Mutex<HashMap<String, KvPair>>>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key.clone(), KvPair::new(key, value));
    }

    pub fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(&key).map(|kv_pair| kv_pair.value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_store_basic_insertion() {
        let kv_store = KvStore::new();
        kv_store.set("key1".to_string(), "value1".to_string());
    
        assert_eq!(kv_store.get("key1".to_string()), Some("value1".to_string()));
        assert_eq!(kv_store.get("key2".to_string()), None);
    }

    #[test]
    fn test_kv_store_extreme_ram_usage() {
        let kv_store = KvStore::new();
        let num_keys = 10_000_000;

        // this loop takes up approximately 1.6 GB 
        for i in 1..=num_keys {
            kv_store.set(i.to_string(), (i+1).to_string());
            if i % 100_000 == 0 {
                println!("Inserted {} keys", i);
            }
        }       

        assert_eq!(kv_store.get("100000".to_string()), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000".to_string()), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000".to_string()), Some("1000001".to_string()));
    }
}