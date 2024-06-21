#[allow(unused_imports)]
use std::io::{self, Write};

mod command;
mod parser;

fn main() {
    let path_env = parser::get_path();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        parser::parse_command(&input, path_env.clone());
    }
}
