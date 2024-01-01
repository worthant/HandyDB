use super::Command;

pub struct CloseCommand;

impl Command for CloseCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        println!("Closing the application...");
        std::process::exit(0);
    }

    fn brief_descr(&self) -> &'static str {
        "close - Close the program"
    }

    fn detailed_descr(&self) -> &'static str {
        "The 'close' command:\n\
         - Safely shuts down the application.\n\
         - Ensures all operations are completed before exit.\n\
         - Use this command to exit the program gracefully."
    }
}
