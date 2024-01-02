use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
mod cli;
use cli::CommandManager;

fn main() {
    println!("Hello, world from {}! I was compiled on {}.", CURRENT_PLATFORM, COMPILED_ON);
    let mut manager = CommandManager::new();
    manager.setup_commands();
    manager.run();
}