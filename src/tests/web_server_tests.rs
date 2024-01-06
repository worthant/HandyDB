use reqwest;
use futures::future::join_all;

#[tokio::test]
async fn test_multiple_set_requests() {
    let client = reqwest::Client::new();

    let requests = (0..10000).map(|i| {
        let client = client.clone();
        tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            let res = client.post("http://127.0.0.1:8080/set")
                .json(&serde_json::json!({ "key": key, "value": value }))
                .send()
                .await;
            res.unwrap().text().await.unwrap()
        })
    });

    let results = join_all(requests).await;
    for result in results {
        assert_eq!(result.unwrap(), "Value set successfully");
    }
}

#[tokio::test]
async fn test_multiple_get_requests() {
    let client = reqwest::Client::new();

    let requests = (0..10000).map(|i| {
        let client = client.clone();
        tokio::spawn(async move {
            let key = format!("key{}", i);
            let res = client.get(format!("http://127.0.0.1:8080/get/{}", key))
                .send()
                .await;
            res.unwrap().text().await.unwrap()
        })
    });

    let start_time = std::time::Instant::now();
    let results = join_all(requests).await;
    let duration = start_time.elapsed();

    for result in results {
        assert!(result.is_ok());
    }

    println!("Time taken for 40,000 get requests: {:?}", duration);
}
