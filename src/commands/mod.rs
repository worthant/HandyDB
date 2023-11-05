pub mod help;
pub mod info;
pub mod close;
pub mod rand;
pub mod history;
pub mod oper;

pub use self::help::HelpCommand;
pub use self::info::InfoCommand;
pub use self::close::CloseCommand;
pub use self::rand::RandomCommand;
pub use self::oper::OperationsCommand;
// pub use self::history::HistoryCommand;

pub trait Command {
    fn execute(&self);
    fn help(&self) -> String;
}