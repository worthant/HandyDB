use super::Command;

pub struct CloseCommand;

impl Command for CloseCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        println!("Closing the application...");
        std::process::exit(0);
    }

    fn help(&self) -> String {
        "The 'close' command shuts down the application safely.".to_string()
    }
}
