#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.contains("exit") {
            return ExitCode::from(0);
        } else if command.contains("echo") {
            println!("{}", str::replace(&command, "echo", ""))
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
