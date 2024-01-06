use super::Command;
use crate::db::KvStore;
use std::sync::Arc;
use std::time::{Duration, Instant};
use reqwest::Client;
use tokio::time::sleep;
use tokio::sync::mpsc;
use tokio::spawn;


// Standalone asynchronous function for set operation over HTTP
async fn set_operation_http(key: String, value: String) -> reqwest::Result<()> {
    let client = Client::new();
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
                    let mut durations = Vec::new();
                    let (tx, mut rx) = mpsc::channel::<()>(num);

                    for i in 0..num {
                        let key = format!("key{}", i);
                        let value = format!("value{}", i);
                        let tx_clone = tx.clone();
                        
                        let duration = Self::measure(|| {
                            tokio::spawn(async move {
                                match set_operation_http(key, value).await {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Error during set operation: {:?}", e);
                                        let _ = tx_clone.send(()).await; // send an error signal
                                    }
                                }
                            });
                        });
                        durations.push(duration);
                    }

                    drop(tx); // Close the channel

                    // Count errors
                    let mut errors = 0;
                    while rx.recv().await.is_some() {
                        errors += 1;
                    }

                    // Output total and average time
                    let total_duration: Duration = durations.iter().sum();
                    let avg_duration = total_duration / num as u32;
                    println!("Total time: {:?}", total_duration);
                    println!("Average time per set operation: {:?}", avg_duration);
                    println!("Number of errors: {}", errors);
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
