use super::Command;
use crate::db::KvStore;
use std::sync::{Arc, Mutex};

pub struct SetCommand {
    kv_store: Arc<Mutex<KvStore>>,
}

impl SetCommand {
    pub fn new(kv_store: Arc<Mutex<KvStore>>) -> Self {
        SetCommand { kv_store }
    }

    pub fn brief_descr() -> &'static str {
        "set - Set a value in the key-value store"
    }

    pub fn detailed_descr() -> &'static str {
        "The 'set' command stores a value associated with a key in the key-value store."
    }
}

impl Command for SetCommand {
    fn execute(&self, args: Option<&[&str]>) {
        if let Some(args) = args {
            if args.len() == 2 {
                let key = args[0].to_string();
                let value = args[1].to_string();
                let kv_store = self.kv_store.lock().unwrap();
                kv_store.set(key, value);
                println!("Value set successfully");
            } else {
                println!("Usage: set <key> <value>");
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
