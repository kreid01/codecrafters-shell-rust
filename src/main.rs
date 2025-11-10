#[allow(unused_imports)]
use std::io::{self, Write};
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
        }
    }
}

fn get_type(command: String) {
    let t = command.replacen("type ", "", 1);
    if t.starts_with("type") || t.starts_with("echo") || t.starts_with("exit") {
        println!("{} is a shell builtin", t.trim());
    } else {
        not_found(t)
    }
}

fn not_found(command: String) {
    println!("{}: command not found", command.trim());
}
