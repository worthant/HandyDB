use std::{sync::Arc, thread};
use actix_web::{HttpServer, App, web};
use cli::CommandManager;
use db::{KvStore, ShardManager};
use crate::network::configure_services;
use utils::logger::Logger;
use clap::{Command, Arg, value_parser, ArgAction};

mod db;
mod cli;
mod models;
mod tests;
mod network;
mod utils;

pub fn create_shared_kv_store() -> Arc<KvStore> {
    Arc::new(KvStore::new())
}

// pub fn create_shared_kv_store(shard_manager: Arc<ShardManager>) -> Arc<KvStore> {
//     Arc::new(KvStore::new(shard_manager))
// }

fn main() {
    let matches = Command::new("HandyDB")
        .version("1.1.1")
        .author("worthant")
        .about("HandyDB - Distributed In-Memory Key-Value Store")
        .arg(Arg::new("port")
             .long("port")
             .value_parser(clap::value_parser!(String))
             .default_value("8080")
             .help("Port to bind the server"))
        .arg(Arg::new("shard-id")
             .long("shard-id")
             .value_parser(clap::value_parser!(String))
             .default_value("1")
             .help("Unique identifier for this shard"))
        .get_matches();

    let port = matches.get_one::<String>("port").unwrap().clone();
    let shard_id = matches.get_one::<String>("shard-id").unwrap().clone();

    let logger = Logger::new();
    logger.log_platform_info();

    // Initialize Shard Manager here
    // let shard_manager = Arc::new(ShardManager::new(shard_id.clone()));

    let kv_store = create_shared_kv_store();

    // Clone KvStore for CLI
    let cli_kv_store = kv_store.clone();

    // Run CLI in a separate thread
    thread::spawn(move || {
        let mut manager = CommandManager::new();
        manager.setup_commands(cli_kv_store);
        manager.run();
    });

    // New Tokio runtime
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(20) // number of threads
        .enable_all()
        .build()
        .unwrap();

    match runtime.block_on(async_main(kv_store.clone(), port.clone())) {
        Ok(_) => println!("Tokio runtime created"),
        Err(e) => eprintln!("Error occurred: {:?}", e),
    }
}

pub async fn async_main(kv_store: Arc<KvStore>, port: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kv_store.clone()))
            .configure(configure_services)
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
