use std::process::Command;

use crate::parser;

pub enum BuiltInCommand {
    Cat,
    Echo,
}

impl BuiltInCommand {
    fn as_str(&self) -> &str {
        match self {
            BuiltInCommand::Cat => "cat",
            BuiltInCommand::Echo => "echo",
        }
    }
}

pub fn execute_command(
    command: BuiltInCommand,
    path: &String,
    args: Vec<String>,
) -> Option<String> {
    let input = match Command::new(command.as_str()).args(&args).output() {
        Ok(output) => output,
        Err(_) => {
            println!("{}: Command not found", path.trim());
            return None;
        }
    };

    return Some(String::from_utf8_lossy(&input.stdout).trim().to_string());
}

pub fn execute(command: &str) {
    let (exe, args) = parser::parse_execute_command(command);

    let input = match Command::new(exe.trim()).args(args).output() {
        Ok(output) => output,
        Err(_) => {
            println!("{}: command not found", command.trim());
            return;
        }
    };

    let output = String::from_utf8_lossy(&input.stdout);
    print!("{}", output);
}
