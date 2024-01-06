#[cfg(test)]
mod tests {
    use crate::db::KvStore;
    use crate::network::configure_services;

    use actix_web::{HttpServer, App, web};
    use reqwest::blocking::Client;
    use std::time::Instant;
    use std::sync::Arc;

    fn create_shared_kv_store() -> Arc<KvStore> {
        Arc::new(KvStore::new())
    }

    #[test]
    fn test_server_responses() {
        let kv_store = create_shared_kv_store();

        // Create a new Tokio runtime
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(16) // You can adjust the number of threads
            .enable_all()
            .build()
            .unwrap();

        match runtime.block_on(async_main(kv_store.clone())) {
            Ok(_) => println!("Tokio runtime created"),
            Err(e) => eprintln!("Error occured: {:?}", e),
        }

        // Создание HTTP-клиента
        let client = Client::new();

        let start_time = Instant::now();

        for i in 1..=2 {
            let key = format!("key{}", i);
            let value = format!("value{}", i);

            let res = client.post("http://127.0.0.1:8080/set")
                .json(&serde_json::json!({ "key": key, "value": value }))
                .send()
                .unwrap();

            assert!(res.status().is_success());
        }

        let duration = start_time.elapsed();
        println!("Time taken for 5000 set requests: {:?}", duration);
    }

    pub async fn async_main(kv_store: Arc<KvStore>) -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(kv_store.clone()))
            .configure(configure_services)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
    }
}
