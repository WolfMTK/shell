use std::path::PathBuf;
use std::io;
use std::io::Write;
use crate::command;

pub fn get_path() -> Vec<PathBuf> {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(":")
        .map(PathBuf::from)
        .collect::<Vec<_>>()
}


fn tokenize(input: &str) -> (&str, &str) {
    input.split_once(" ").unwrap_or((input, ""))
}

pub fn parse_command(input: &str, path_buf: Vec<PathBuf>) {
    let command = input.trim();

    if command == "exit" {
        command::exit_command(0);
    }

    let (command, args) = tokenize(command);

    match command {
        "exit" => command::exit_command(args.parse::<i32>().unwrap()),
        "echo" => command::echo_command(args),
        "type" => command::type_command(&path_buf, args),
        "pwd" => command::pwd_command(),
        "cd" => command::cd_command(args),
        _ => {
            if let Some(command) = command::find_command(command, &path_buf) {
                let output = std::process::Command::new(command)
                    .args(args.split_whitespace())
                    .output()
                    .unwrap();
                io::stdout().write_all(&output.stdout).unwrap();
            } else {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
