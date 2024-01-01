use super::Command;

pub struct InfoCommand;

impl Command for InfoCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        println!("Project Name: HandyDB");
        println!("Author: Dvorkin Boris a.k.a worthant");
        println!("Date of Initialization: <date>");
        println!("Size of Memory: <size>");
    }

    fn help(&self) -> String {
        "The 'info' command displays detailed information about the project, such as its name, author, initialization date, memory usage, and more.".to_string()
    }
}