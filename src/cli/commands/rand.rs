use super::Command;
use crate::cli::utils::generate_random_number;
pub struct RandomCommand;

impl Command for RandomCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        generate_random_number();
    }

    fn help(&self) -> String{
        "Generates random number for fun and displays it to the console".to_string()
    }
}   