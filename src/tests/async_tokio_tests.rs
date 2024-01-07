// 3.5s for 5_000 'set' requests
#[tokio::test]
async fn test_multiple_set_requests() {
    use reqwest;
    use futures::future::join_all;

    let client = reqwest::Client::new();

    let requests = (0..5000).map(|i| {
        let client = client.clone();    
        tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            let res = client.post("http://127.0.0.1:8080/set")
                .json(&serde_json::json!({ "key": key, "value": value }))
                .send()
                .await;
           
            match res {
                Ok(response) => response.text().await.unwrap_or_else(|_| "Error".to_string()),
                Err(_) => "Error".to_string(),
            }
        })
    });

    let results = join_all(requests).await;
    for result in results {
        assert_eq!(result.unwrap(), "Value set successfully");
    }
}


// 5s for 10_000 'set' requests
#[tokio::test]
async fn test_multiple_set_requests_semaphore() {
    use std::sync::Arc;
    use reqwest;
    use futures::future::join_all;
    use tokio::sync::Semaphore;
    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(5000));

    let mut handles = Vec::new();

    for i in 0..10_000 {
        let client = client.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let handle = tokio::spawn(async move {
            let key = format!("key{}", i);
            let value = format!("value{}", i);
            let res = client.post("http://127.0.0.1:8080/set")
                .json(&serde_json::json!({ "key": key, "value": value }))
                .send()
                .await;

            drop(permit);
            
            match res {
                Ok(response) => response.text().await.unwrap_or_else(|_| "Error".to_string()),
                Err(_) => "Error".to_string(),
            }
        });

        handles.push(handle);
    }

    let results = join_all(handles).await;
    for result in results {
        assert_eq!(result.unwrap(), "Value set successfully");
    }
}


// 3.5s for 5_000 'get' requests
#[tokio::test]
async fn test_multiple_get_requests() {
    use futures::future::join_all;
    let client = reqwest::Client::new();

    let requests = (0..5000).map(|i| {
        let client = client.clone();
        tokio::spawn(async move {
            let key = format!("key{}", i);
            let res = client.get(format!("http://127.0.0.1:8080/get/{}", key))
                .send()
                .await;
            
            match res {
                Ok(response) => response.text().await.unwrap_or_else(|_| "Error".to_string()),
                Err(_) => "Error".to_string(),
            }
        })
    });

    let start_time = std::time::Instant::now();
    let results = join_all(requests).await;
    let duration = start_time.elapsed();

    for result in results {
        assert!(result.is_ok());
    }

    println!("Time taken for 5000 get requests: {:?}", duration);
}
