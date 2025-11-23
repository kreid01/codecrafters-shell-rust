use std::io::{self, Write};
use std::process::ExitCode;

use crate::cd::pwd;
use crate::exe::get_exe_path;
use crate::utils::input_handler::{handle_input, InputResult};

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod exe;
mod executor;
mod ls;
mod utils;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() -> ExitCode {
    loop {
        print!("\r$ ");
        io::stdout().flush().unwrap();

        let buffer: String;

        match handle_input() {
            InputResult::Completed(input) => {
                buffer = input;
            }
            InputResult::Exit(code) => return ExitCode::from(code),
        }

        if buffer.is_empty() {
            continue;
        }

        match buffer {
            cmd if cmd.starts_with("exit") => {
                return ExitCode::from(0);
            }
            cmd if cmd.starts_with("echo") => echo::echo(cmd),
            cmd if cmd.starts_with("type") => execute_type(cmd),
            cmd if cmd.starts_with("pwd") => pwd(),
            cmd if cmd.starts_with("cd") => cd::cd(cmd.as_str()),
            cmd if cmd.starts_with("cat") => cat::cat(cmd.as_str()),
            cmd if cmd.starts_with("ls") => ls::ls(cmd),
            _ => executor::execute(&buffer),
        }
    }
}

pub fn execute_type(command: String) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}
