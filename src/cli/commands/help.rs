use super::Command;
use crate::cli::utils::print_in_box;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, args: Option<&[&str]>) {
        let mut commands = vec![
            "help - Display this help message",
            "info - Display information about the project",
            "rand - Generate a random number",
            "operations - Perform arithmetic operations",
            "close - Close the program",
        ];

        print_in_box("help", &commands);
    }

    fn help(&self) -> String {
        "The 'help' command displays a list of available commands and their brief descriptions.".to_string()
    }
}
