use super::Command;
use crate::db::KvStore;
use std::sync::Arc;

pub struct GetCommand {
    kv_store: Arc<KvStore>,
}

impl GetCommand {
    pub fn new(kv_store: Arc<KvStore>) -> Self {
        GetCommand { kv_store }
    }

    pub fn brief_descr() -> &'static str {
        "get - Retrieve a value from the key-value store"
    }

    pub fn detailed_descr() -> &'static str {
        "The 'get' command retrieves a value associated with a key from the key-value store."
    }
}

impl Command for GetCommand {
    fn execute(&self, args: Option<&[&str]>) {
        if let Some(args) = args {
            if args.len() == 1 {
                let key = args[0].to_string();
                match self.kv_store.get(&key) {
                    Some(value) => println!("Value: {}", value),
                    None => println!("Key not found"),
                }
            } else {
                println!("Usage: get <key>");
            }
        }
    }

    fn brief_descr(&self) -> &'static str {
        Self::brief_descr()
    }

    fn detailed_descr(&self) -> &'static str {
        Self::detailed_descr()
    }
}
