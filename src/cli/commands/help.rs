use super::{Command, InfoCommand, RandomCommand, OperationsCommand, CloseCommand};
use crate::cli::utils::print_in_box;
use std::collections::HashSet;

pub struct HelpCommand;

impl Command for HelpCommand {
    fn execute(&self, args: Option<&[&str]>) {
        let command_descriptions = vec![
            ("help", HelpCommand.brief_descr(), HelpCommand.detailed_descr()),
            ("info", InfoCommand.brief_descr(), InfoCommand.detailed_descr()),
            ("rand", RandomCommand.brief_descr(), RandomCommand.detailed_descr()),
            ("oper", OperationsCommand.brief_descr(), OperationsCommand.detailed_descr()),
            ("close", CloseCommand.brief_descr(), CloseCommand.detailed_descr())
            // ... commands for the kv_store, network and other things here in the future
        ];

        match args {
            Some(args) if !args.is_empty() => {
                let unique_args: HashSet<_> = args.iter().collect();
                for arg in unique_args {
                    if let Some((_, _, detailed)) = command_descriptions.iter().find(|&&(cmd, _, _)| cmd == *arg) {
                        print_in_box(arg, &[detailed]);
                    } else {
                        print_in_box(arg, &["No detailed help available for this command."]);
                    }
                }
            }
            _ => {
                let brief_descriptions: Vec<&str> = command_descriptions.iter().map(|&(_, brief, _)| brief).collect();
                print_in_box("Available Commands", &brief_descriptions);
            }
        }
    }

    fn brief_descr(&self) -> &'static str {
        "help - Displays help information."
    }

    fn detailed_descr(&self) -> &'static str {
        "The 'help' command displays a list of available commands and their brief descriptions."
    }
}

