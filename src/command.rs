use std::path::{Path, PathBuf};

pub fn exit_command(code: i32) {
    std::process::exit(code);
}

pub fn pwd_command() {
    let current_dir = std::env::current_dir().unwrap();
    println!("{}", current_dir.display());
}

pub fn find_command(command: &str, path: &[PathBuf]) -> Option<PathBuf> {
    if let Some(path) =
        path.iter().find(|dir| dir.join(command).is_file()) {
        Some(path.join(command))
    } else {
        None
    }
}

pub fn type_command(path_buf: &Vec<PathBuf>, args: &str) {
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


pub fn echo_command(args: &str) {
    println!("{args}")
}

pub fn cd_command(args: &str) {
    let path = Path::new(args);
    if path.as_os_str() == "~" {
        let home = std::env::var("HOME").unwrap();
        let path = Path::new(&home);
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
