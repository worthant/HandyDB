mod cli;
use cli::CommandManager;

fn main() {
    let mut manager = CommandManager::new();
    manager.setup_commands();
    manager.run();
}