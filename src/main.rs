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
    } else {
        check_dirs(&command)
    }
}

fn check_dirs(command: &String) {
    if let Ok(paths) = env::var("PATH") {
        let t = command.replacen("type ", "", 1);

        for dir in env::split_paths(&paths) {
            let file = dir.join(command);
            if file.is_file() {
                if let Ok(meta) = file.metadata() {
                    println!("{}", file.to_string_lossy());
                    if meta.permissions().mode() & 0o111 != 0 && file.to_string_lossy() == t {
                        println!("{} is {}", t.trim(), dir.display());
                    }
                }
            }
        }
    }
}

fn not_found(command: String) {
    println!("{}: command not found", command.trim());
}

fn command_not_found(command: String) {
    println!("{}: not found", command.trim());
}
