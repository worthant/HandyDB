use std::{sync::{Arc, Mutex}, thread};
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

fn create_shared_kv_store() -> Arc<Mutex<KvStore>> {
    Arc::new(Mutex::new(KvStore::new()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello from {}! This HandyDB release was compiled for {}.", CURRENT_PLATFORM, COMPILED_ON);

    // Enable logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

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

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(kv_store.clone()))
        .configure(configure_services)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}