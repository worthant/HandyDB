use super::Command;
use crate::db::KvStore;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use reqwest::Client;
use tokio::sync::mpsc;


// Standalone asynchronous function for set operation over HTTP
async fn set_operation_http(client: &Client, key: String, value: String) -> reqwest::Result<()> {
    let set_request = serde_json::json!({ "key": key, "value": value });

    client.post("http://127.0.0.1:8080/set")
          .json(&set_request)
          .send()
          .await?
          .error_for_status()?;
    Ok(())
}

pub struct TestCommand {
    kv_store: Arc<KvStore>,
}

impl TestCommand {
    pub fn new(kv_store: Arc<KvStore>) -> Self {
        TestCommand { kv_store }
    }

    // Measure execution time of a block of code
    fn measure<F: FnOnce()>(f: F) -> Duration {
        let start = Instant::now();
        f();
        start.elapsed()
    }

    // Perform set operation
    fn set_operation(&self, key: &str, value: &str) {
        self.kv_store.set(key.to_string(), value.to_string());
    }

    // Execute the test command
    pub async fn execute_test(&self, args: Option<&[&str]>) {
        if let Some(args) = args {
            match args {
                ["set", num, "http"] => {
                    let num: usize = match num.parse() {
                        Ok(n) if n > 0 => n,
                        _ => {
                            eprintln!("Invalid number of operations. Please provide a positive integer.");
                            return;
                        }
                    };

                    let batch_size = 200; // requests per Client
                    let num_clients = (num as f64 / batch_size as f64).ceil() as usize;
                    let mut clients = Vec::with_capacity(num_clients);
                    let error_counts = Arc::new(tokio::sync::Mutex::new(HashMap::<String, usize>::new()));

                    for _ in 0..num_clients {
                        clients.push(Client::new());
                    }

                    let mut durations = Vec::new();
                    let (tx, _rx) = mpsc::channel::<()>(num);

                    for (i, client) in clients.into_iter().enumerate() {
                        let start_index = i * batch_size;
                        let end_index = ((i + 1) * batch_size).min(num);
                        let client_ref = Arc::new(client);
                        let error_counts_ref = error_counts.clone();

                        for _j in start_index..end_index {

                            let key = format!("key{}", i);
                            let value = format!("value{}", i);
                            let tx_clone = tx.clone();
                            let client_clone = client_ref.clone();
                            let error_counts_ref = error_counts_ref.clone();
                            
                            let duration = Self::measure(|| {
                                tokio::spawn(async move {
                                    match set_operation_http(&client_clone, key, value).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            let mut errors = error_counts_ref.lock().await;
                                            *errors.entry(e.to_string()).or_insert(0) += 1;
                                            let _ = tx_clone.send(()).await;
                                        }
                                    }
                                });
                            });
                            durations.push(duration);
                        }
                    }


                    drop(tx); // Close the channel

                    // Output total and average time
                    let total_duration: Duration = durations.iter().sum();
                    let avg_duration = total_duration / num as u32;
                    println!("Total time: {:?}", total_duration);
                    println!("Average time per set operation: {:?}", avg_duration);

                    // Print error summary
                    let errors = error_counts.lock().await;
                    println!("Number of different errors: {}", errors.len());
                    for (error, count) in errors.iter() {
                        println!("Error: {:?}", error);
                        println!("Occurences: {}", count);
                    }
                    
                }
                ["set", num] => {
                    let num: usize = match num.parse() {
                        Ok(n) if n > 0 => n,
                        _ => {
                            eprintln!("Invalid number of operations. Please provide a positive integer.");
                            return;
                        }
                    };
                    let mut durations = Vec::new();

                    for i in 0..num {
                        let key = format!("key{}", i);
                        let value = format!("value{}", i);
                        let duration = Self::measure(|| self.set_operation(&key, &value));
                        durations.push(duration);
                    }

                    // Output total and average time
                    let total_duration: Duration = durations.iter().sum();
                    let avg_duration = total_duration / num as u32;
                    println!("Total time: {:?}", total_duration);
                    println!("Average time per set operation: {:?}", avg_duration);
                }
                _ => println!("Usage: test set <number of operations> [http]"),
            }
        } else {
            println!("Usage: test set <number of operations> [http]");
        }
    }
}

// Implement Command trait for TestCommand
impl Command for TestCommand {
    fn execute(&self, args: Option<&[&str]>) {
        tokio::runtime::Runtime::new().unwrap().block_on(self.execute_test(args));
    }

    fn brief_descr(&self) -> &'static str {
        "Test performance of set/get operations"
    }

    fn detailed_descr(&self) -> &'static str {
        "Usage: test set <number of operations> [http]"
    }
}
