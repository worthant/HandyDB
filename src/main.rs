use command_manager::CommandManager;
mod command_manager;
mod commands;
mod utils;

fn main() {
    let mut manager = CommandManager::new();
    manager.setup_commands();
    manager.run();
}