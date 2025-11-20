use std::env::{self};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::ExitCode;

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod executor;
mod ls;
mod utils;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.split_whitespace().nth(0).unwrap() == "exit" {
            return ExitCode::from(0);
        }

        match command.split_whitespace().nth(0).unwrap() {
            "echo" => echo::echo(command),
            "type" => execute_type(&command),
            "pwd" => pwd(),
            "cd" => cd::cd(&command),
            "cat" => cat::cat(&command),
            "ls" => ls::ls(command),
            _ => executor::execute(&command),
        };
    }
}

fn pwd() {
    let curr_dir = env::current_dir().unwrap();
    println!("{}", curr_dir.display())
}

pub fn execute_type(command: &str) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}

fn get_exe_path(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 != 0 {
                        return Some(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    None
}
