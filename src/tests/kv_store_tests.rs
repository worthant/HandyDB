#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use rayon::prelude::*;
    use threadpool::ThreadPool;

    use crate::db::KvStore;

    #[test]
    fn test_concurrent_threadpool_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;
        let num_threads = 500;
        let pool = ThreadPool::new(num_threads);

        let start_time = std::time::Instant::now();

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

        println!("Concurrent threadpool 'set' test: {:?}", start_time.elapsed());
    }

    #[test]
    fn test_concurrent_rayon_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        let start_time = std::time::Instant::now();

        (1..=num_keys).into_par_iter().for_each(|i| {
            let kv_store = kv_store.clone();
            kv_store.set(i.to_string(), (i + 1).to_string());
        });

        assert_eq!(kv_store.get("100000"), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000"), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000"), Some("1000001".to_string()));

        println!("Concurrent rayon 'set' test: {:?}", start_time.elapsed());
    }

    #[test]
    fn test_kv_store_extreme_ram_usage() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;

        let start_time = std::time::Instant::now();

        // this loop takes up approximately 1.6 GB
        for i in 1..=num_keys {
            let kv_store = kv_store.clone();
            kv_store.set(i.to_string(), (i + 1).to_string());
        }

        assert_eq!(kv_store.get("100000"), Some("100001".to_string()));
        assert_eq!(kv_store.get("500000"), Some("500001".to_string()));
        assert_eq!(kv_store.get("1000000"), Some("1000001".to_string()));

        println!("Non-concurrent 'set' test: {:?}", start_time.elapsed());
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
            let kv_store = kv_store.clone();
            assert_eq!(kv_store.get(&i.to_string()), Some((i + 1).to_string()));
        });
        let duration = start_time.elapsed();
        println!("Concurrent rayon `get` test: {:?}", duration);
    }

    #[test]
    fn test_concurrent_threadpool_kv_store_get_performance() {
        let kv_store = Arc::new(KvStore::new());
        let num_keys = 1_000_000;
        let num_threads = 100;
        let pool = ThreadPool::new(num_threads);

        // Заполнение KvStore ключами
        (1..=num_keys).into_par_iter().for_each(|i| {
            let kv_store = kv_store.clone();
            kv_store.set(i.to_string(), (i + 1).to_string());
        });

        let start_time = std::time::Instant::now();

        for i in 1..=num_keys {
            let kv_store = kv_store.clone();
            pool.execute(move || {
                assert_eq!(kv_store.get(&i.to_string()), Some((i + 1).to_string()));
            });
        }

        pool.join();

        println!("Concurrent threadpool 'get' test: {:?}", start_time.elapsed());
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
            let kv_store = kv_store.clone();
            assert_eq!(kv_store.get(&i.to_string()), Some((i + 1).to_string()));
        }
        let duration = start_time.elapsed();
        println!("Non-concurrent get time: {:?}", duration);
    }

    #[test]
    fn test_kv_store_basic_insertion() {
        let kv_store = KvStore::new();

        let start_time = std::time::Instant::now();
        kv_store.set("key1".to_string(), "value1".to_string());

        assert_eq!(kv_store.get("key1"), Some("value1".to_string()));
        assert_eq!(kv_store.get("key2"), None);

        println!("One 'set' operation test: {:?}", start_time.elapsed());
    }
}
