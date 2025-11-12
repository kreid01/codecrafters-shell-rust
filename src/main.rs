use std::env;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::ExitCode;

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
            get_type(command);
        } else {
            not_found(command)
        }
    }
}

fn get_type(command: String) {
    let t = command.replacen("type ", "", 1);
    if t.starts_with("type") || t.starts_with("echo") || t.starts_with("exit") {
        println!("{} is a shell builtin", t.trim());
    } else if let Some(path) = is_command_in_path(&command) {
        println!("{} is {}", command, path)
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

fn not_found(command: String) {
    println!("{}: command not found", command.trim());
}
