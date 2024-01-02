use super::Command;

pub struct InfoCommand;

impl Command for InfoCommand {
    fn execute(&self, _args: Option<&[&str]>) {
        // TODO: add implementation for showcasing info about kv-store and the database overall
        println!("Project Name: HandyDB");
        println!("Author: Dvorkin Boris a.k.a worthant");
        println!("Date of Initialization: <date>");
        println!("Size of Memory: <size>");
    }

    fn brief_descr(&self) -> &'static str {
        "info - Display information about the project"
    }

    fn detailed_descr(&self) -> &'static str {
        "The 'info' command provides detailed information about the HandyDB project:\n\
         - Project Name: Displays the name of the project.\n\
         - Author: Shows the name of the project's author.\n\
         - Initialization Date: The date when the project was initiated.\n\
         - Memory Usage: Current memory usage statistics of the application."
    }
}