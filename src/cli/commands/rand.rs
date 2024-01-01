use super::Command;
use crate::cli::utils::generate_random_number;
pub struct RandomCommand;

impl Command for RandomCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        generate_random_number();
    }

    // fn help(&self) -> String{
    //     "Generates random number for fun and displays it to the console".to_string()
    // }

    fn brief_descr(&self) -> &'static str {
        "rand - Generate a random number"
    }

    fn detailed_descr(&self) -> &'static str {
        "The 'rand' command:\n\
         - Generates a random number each time it's called.\n\
         - Useful for testing random number generation in Rust.\n\
         - Displays the generated number to the console."
    }
}   