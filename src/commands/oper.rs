use super::Command;
use crate::utils::operations;
use std::io::{self, Write};

pub struct OperationsCommand;

impl Command for OperationsCommand {
    fn execute(&self) {
        println!("Please, enter a and b!");

        let a = loop {
            print!("a: ");
            io::stdout().flush().unwrap();
            let mut a_str = String::new();
            io::stdin()
                .read_line(&mut a_str)
                .expect("Failed to read line");
            match a_str.trim().parse::<i32>() {
                Ok(num) => break num, // Successfully parsed an i32, break the loop
                Err(_) => println!("This is not a valid number. Please enter an integer."),
            }
        };

        let b = loop {
            print!("b: ");
            io::stdout().flush().unwrap();
            let mut b_str = String::new();
            io::stdin()
                .read_line(&mut b_str)
                .expect("Failed to read line");
            match b_str.trim().parse::<i32>() {
                Ok(num) => {
                    if num != 0 {
                        break num; // Successfully parsed an i32 that's not zero, break the loop
                    } else {
                        println!("b cannot be zero. Please enter a non-zero integer.");
                    }
                },
                Err(_) => println!("This is not a valid number. Please enter an integer."),
            }
        };

        operations(a, b);
    }

    fn help(&self) -> String {
        "Showcases Rust's operations features, such as addition, subtraction, multiplication, division with and without remainder.".to_string()
    }
}