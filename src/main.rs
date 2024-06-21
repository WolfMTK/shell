#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn exit_command(code: i32) {
    std::process::exit(code);
}

fn find_command(command: &str, path: &[PathBuf]) -> Option<PathBuf> {
    if let Some(path) =
        path.iter().find(|dir| dir.join(command).is_file()) {
        Some(path.join(command))
    } else {
        None
    }
}

fn pwd_command() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

fn type_command(path_buf: &Vec<PathBuf>, args: &str) {
    match args {
        "echo" | "exit" | "type" | "pwd" | "cd" => {
            println!("{args} is a shell builtin")
        }
        _ => if let Some(command) = find_command(args, &path_buf) {
            println!("{} is {}", args, command.display());
        } else {
            println!("{args}: not found");
        }
    }
}

fn get_path() -> Vec<PathBuf> {
    std::env::var("PATH")
        .unwrap_or_default()
        .split(":")
        .map(PathBuf::from)
        .collect::<Vec<_>>()
}


fn tokenize(input: &str) -> (&str, &str) {
    input.split_once(" ").unwrap_or((input, ""))
}


fn echo_command(args: &str) {
    println!("{args}")
}

fn cd_command(args: &str) {
    let path = Path::new(args);
    if path.as_os_str() == "~" {
        let home = std::env::var("HOME").unwrap();
        let path  = Path::new(&home);
        set_current_dir(args, path);
    } else {
        set_current_dir(args, path);
    }
}

fn set_current_dir(args: &str, path: &Path) {
    if std::env::set_current_dir(path).is_err() {
        println!("cd: {args}: No such file or directory")
    }
}

fn parse_command(input: &str, path_buf: Vec<PathBuf>) {
    let command = input.trim();

    if command == "exit" {
        exit_command(0);
    }

    let (command, args) = tokenize(command);

    match command {
        "exit" => exit_command(args.parse::<i32>().unwrap()),
        "echo" => echo_command(args),
        "type" => type_command(&path_buf, args),
        "pwd" => pwd_command(),
        "cd" => cd_command(args),
        _ => {
            if let Some(command) = find_command(command, &path_buf) {
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

fn main() {
    let path_env = get_path();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        parse_command(&input, path_env.clone());
    }
}
