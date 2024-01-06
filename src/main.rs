use std::{sync::Arc, thread};
use actix_web::{HttpServer, App, web};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use cli::CommandManager;
use db::KvStore;
use crate::network::configure_services;

mod db;
mod cli;
mod models;
mod tests;
mod network;

pub fn create_shared_kv_store() -> Arc<KvStore> {
    Arc::new(KvStore::new())
}

fn main() {
    println!("Hello from {}! This HandyDB release was compiled for {}.", CURRENT_PLATFORM, COMPILED_ON);

    // Enable logging
    // std::env::set_var("RUST_LOG", "debug");
    // env_logger::init();

    // Init our first and last HandyDB instance of this application
    let kv_store = create_shared_kv_store();

    // CLone KvStore for CLI 
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

    match runtime.block_on(async_main(kv_store.clone())) {
        Ok(_) => println!("Tokio runtime created"),
        Err(e) => eprintln!("Error occured: {:?}", e),
    }

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