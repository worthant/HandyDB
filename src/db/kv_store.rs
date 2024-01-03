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

#[cfg(test)]
mod tests {
    use super::*;
    use rayon::prelude::*;
    use threadpool::ThreadPool;

    #[test]
    fn test_concurrent_threadpool_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;
        let num_threads = 4; // Вы можете настроить количество потоков
        let pool = ThreadPool::new(num_threads);

        for i in 1..=num_keys {
            let kv_store = kv_store.clone();
            pool.execute(move || {
                kv_store.set(i.to_string(), (i + 1).to_string());
            });
        }

        pool.join();

        assert_eq!(kv_store.get("100000"), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000"), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000"), Some("1000001".to_string()));
    }

    #[test]
    fn test_concurrent_rayon_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        (1..=num_keys).into_par_iter().for_each(|i| {
            kv_store.set(i.to_string(), (i + 1).to_string());
        });

        assert_eq!(kv_store.get("100000"), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000"), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000"), Some("1000001".to_string()));
    }

    #[test]
    fn test_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        // this loop takes up approximately 1.6 GB
        for i in 1..=num_keys {
            kv_store.set(i.to_string(), (i + 1).to_string());
        }

        assert_eq!(kv_store.get("100000"), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000"), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000"), Some("1000001".to_string()));
    }

    #[test]
    fn test_concurrent_kv_store_get_performance() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        // Заполнение KvStore ключами
        (1..=num_keys).into_par_iter().for_each(|i| {
            kv_store.set(i.to_string(), (i + 1).to_string());
        });

        let start_time = std::time::Instant::now();
        (1..=num_keys).into_par_iter().for_each(|i| {
            assert_eq!(kv_store.get(&i.to_string()), Some((i + 1).to_string()));
        });
        let duration = start_time.elapsed();
        println!("Concurrent get time: {:?}", duration);
    }

    #[test]
    fn test_kv_store_get_performance() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        // Заполнение KvStore ключами
        (1..=num_keys).into_par_iter().for_each(|i| {
            kv_store.set(i.to_string(), (i + 1).to_string());
        });

        let start_time = std::time::Instant::now();
        for i in 1..=num_keys {
            assert_eq!(kv_store.get(&i.to_string()), Some((i + 1).to_string()));
        }
        let duration = start_time.elapsed();
        println!("Concurrent get time: {:?}", duration);
    }

    #[test]
    fn test_kv_store_basic_insertion() {
        let kv_store = KvStore::new();
        kv_store.set("key1".to_string(), "value1".to_string());

        assert_eq!(kv_store.get("key1"), Some("value1".to_string()));
        assert_eq!(kv_store.get("key2"), None);
    }
}
