use std::sync::{Arc, Mutex};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use cli::CommandManager;
use db::KvStore;

mod db;
mod cli;

fn main() {
    println!("Hello from {}! This HandyDB release was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);

    // Init our first and last HandyDB instance of this application
    let kv_store = Arc::new(Mutex::new(KvStore::new()));

    // Init CLI
    let mut manager = CommandManager::new();
    manager.setup_commands(kv_store);
    manager.run();
}