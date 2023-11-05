use std::collections::HashMap;
use std::io::{self, Write};
use crate::commands::{Command, HelpCommand, InfoCommand, CloseCommand, RandomCommand, OperationsCommand};
use crate::utils::print_in_box;

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
                    if parts.is_empty() {
                        continue;
                    }
                    if parts[0] == "help" {
                        if parts.len() == 1 {
                            self.commands.get("help").unwrap().execute();
                        } else {
                            let mut help_messages = Vec::new();
                            for command in parts.iter().skip(1) {
                                let help_message = match self.commands.get(*command) {
                                    Some(cmd) => cmd.help(),
                                    None => format!("No help available for: {}", command),
                                };
                                help_messages.push(help_message);
                            }
                            print_in_box("Help", &help_messages.iter().map(AsRef::as_ref).collect::<Vec<&str>>());
                        }
                    } else if let Some(cmd) = self.commands.get(parts[0]) {
                        cmd.execute();
                    } else {
                        println!("Unknown command: {}", parts[0]);
                    }
                },
                Err(error) => println!("Error reading input: {}", error),
            }
        }
    }
}    
