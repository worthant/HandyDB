use std::collections::HashMap;
use std::io::{self, Write};
use super::commands::{Command, HelpCommand, InfoCommand, CloseCommand, RandomCommand, OperationsCommand};

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

    pub fn setup_commands(&mut self) {
        self.register_command("help", Box::new(HelpCommand));
        self.register_command("info", Box::new(InfoCommand));
        self.register_command("close", Box::new(CloseCommand));
        self.register_command("rand", Box::new(RandomCommand));
        self.register_command("oper", Box::new(OperationsCommand));
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
