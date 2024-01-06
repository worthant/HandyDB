pub mod help;
pub mod info;
pub mod close;
pub mod rand;
pub mod history;
pub mod oper;
pub mod set;
pub mod get;
pub mod test;

pub use self::help::HelpCommand;
pub use self::info::InfoCommand;
pub use self::close::CloseCommand;
pub use self::rand::RandomCommand;
pub use self::oper::OperationsCommand;
pub use self::set::SetCommand;
pub use self::get::GetCommand;
pub use self::test::TestCommand;

// pub use self::history::HistoryCommand;

pub trait Command {
    fn execute(&self, args: Option<&[&str]>);
    fn brief_descr(&self) -> &'static str;
    fn detailed_descr(&self) -> &'static str;
}