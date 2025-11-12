use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::process::ExitCode;

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let builtin = match command.split_whitespace().nth(0).unwrap() {
            "exit" => ExitCode::from(0),
            "echo" => print!("{}", str::replace(&command, "echo ", "")),
            "type" => execute_type(&command),
            "pwd" => pwd(),
            _ => execute(command),
        };
}

pub fn execute(command: &str) {
    let exe = command.split_whitespace().nth(0).unwrap();

    if let Some(_) = get_exe(&exe) {
        let args = command.split_whitespace().filter(|x| !x.contains(exe));

        let input = match Command::new(&exe).args(args).output() {
            Ok(output) => output,
            Err(_) => {
                println!("{}: command not found", command.trim());
                return;
            }
        };

        let output = String::from_utf8_lossy(&input.stdout);
        print!("{}", output);
    }
}

fn pwd() {
    let curr_dir = path::curr_dir()?;
    println!("{}", curr_dir)
}

fn get_exe(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

pub fn execute_type(command: &str) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = is_command_in_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}

fn is_command_in_path(command: &str) -> Option<String> {
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
