
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct KvStore {
    store: Arc<Mutex<HashMap<String, String>>>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&self, key: String, value: String) {
        let mut store = self.store.lock().unwrap();
        store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let store = self.store.lock().unwrap();
        store.get(&key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_store() {
        let kv_store = KvStore::new();
        kv_store.set("key1".to_string(), "value1".to_string());

        assert_eq!(kv_store.get("key1".to_string()), Some("value1".to_string()));
        assert_eq!(kv_store.get("key2".to_string()), None);
    }
}