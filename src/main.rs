use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
    } else {
        check_dirs(command)
    }
}

fn check_dirs(command: String) {
    let binding = env::var("PATH").unwrap();
    let dirs: Vec<&str> = binding.split(":").collect();

    let t = command.replacen("type ", "", 1);
    let mut found = false;

    for dir in dirs {
        if let Ok(files) = fs::read_dir(dir) {
            for file in files.flatten() {
                if file.file_name().into_string().expect("should be a string") == t {
                    println!("{} is {}", t.trim(), dir.to_string());
                    found = true;
                }
            }
        };
    }

    if !found {
        command_not_found(command)
    }
}

// fn check_dirs(dir: &str) {}

fn not_found(command: String) {
    println!("{}: command not found", command.trim());
}

fn command_not_found(command: String) {
    println!("{}: not found", command.trim());
}
