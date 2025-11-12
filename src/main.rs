use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::process::ExitCode;
use std::{env, string};

const BUILTINS: [&str; 3] = ["exit", "echo", "type"];

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.starts_with("exit") {
            return ExitCode::from(0);
        } else if command.starts_with("echo") {
            print!("{}", str::replace(&command, "echo ", ""))
        } else if command.starts_with("type") {
            execute_type(&command);
        } else {
            execute(&command)
        }
    }
}

pub fn execute(command: &str) {
    let exe = command.split_whitespace().nth(0).unwrap();

    if let Some(_) = get_exe(&exe) {
        let args = command.split_whitespace().filter(|x| !x.contains(exe));

        let input = match Command::new(&exe).args(args).output() {
            Ok(output) => output,
            Err(e) => {
                println!("{}: command not found", command.trim());
                return;
            }
        };

        let output = String::from_utf8_lossy(&input.stdout);
        print!("{}", output);
    }
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
