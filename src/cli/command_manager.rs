use std::collections::HashMap;
use std::io::{self, Write};
use std::sync::Arc;
use crate::db::KvStore;
use super::commands::{Command, HelpCommand, InfoCommand, CloseCommand, RandomCommand, OperationsCommand, SetCommand, GetCommand, TestCommand};

pub struct CommandManager {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register_command(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    // TODO: Implement the following commands for HandyDB
    // set <key> <value> - Sets or updates a value for a given key
    // get <key> - Retrieves the value associated with a given key
    // add <key> <value> - Adds a new key-value pair, errors if key exists
    // append <value> - Appends a value with an auto-generated key
    // delete <key> - Deletes a key-value pair based on the key
    // exists <key> - Checks if a key exists in the database
    // list or keys - Lists all keys in the database
    // clear - Clears all data from the database
    // update <key> <value> - Updates the value for an existing key, errors if key doesn't exist
    // info - Displays information about the database
    // help - Displays help information about commands
    // exit or quit - Exits the database CLI
    // export <filename> - Exports database contents to a file
    // import <filename> - Imports data from a file into the database
    // find <pattern> - Searches for keys matching a pattern
    // history - Shows a history of executed commands (optional)
    // stats - Shows statistics about database usage and performance
    // rename <old_key> <new_key> - Renames a key
    // merge <key1> <key2> <new_key> - Merges two keys into a new key
    pub fn setup_commands(&mut self, kv_store: Arc<KvStore>) {
        self.register_command("help", Box::new(HelpCommand));
        self.register_command("info", Box::new(InfoCommand));
        self.register_command("close", Box::new(CloseCommand));
        self.register_command("rand", Box::new(RandomCommand));
        self.register_command("oper", Box::new(OperationsCommand));
        self.register_command("set", Box::new(SetCommand::new(kv_store.clone())));
        self.register_command("get", Box::new(GetCommand::new(kv_store.clone())));
        self.register_command("test", Box::new(TestCommand::new(kv_store.clone())));
    }

    pub fn run(&mut self) {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let parts: Vec<&str> = input.trim().split_whitespace().collect();
                    if let Some((command_name, args)) = parts.split_first() {
                        match self.commands.get(*command_name) {
                            Some(command) => command.execute(Some(args)),
                            None => println!("Unknown command: {}", command_name),
                        }
                    }
                },
                Err(error) => println!("Error reading input: {}", error),
            }
        }
    }
}    
