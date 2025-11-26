use std::fmt::format;
use std::io::{self, pipe, Write};
use std::process::{CommandArgs, ExitCode};

use crate::cat::CommandResult;
use crate::cd::pwd;
use crate::exe::get_exe_path;
use crate::utils::input_handler::{handle_input, InputResult};
use crate::utils::printer::print_lines;

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod exe;
mod executor;
mod ls;
mod tail;
mod utils;
mod wc;

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

        let piped_commands: Vec<&str> = buffer.split("|").collect();
        let mut result: CommandResult = CommandResult::Output("".to_string());

        for command in piped_commands {
            let mut piped_command: String = command.trim_start().to_string();
            match result {
                CommandResult::Output(s) => {
                    let cmd = format!(" {}", s);
                    piped_command.push_str(&cmd);
                }
                _ => {
                    break;
                }
            }

            result = execute_command(&piped_command);
        }
    }
}

pub fn execute_command(command: &str) -> CommandResult {
    return match command {
        cmd if cmd.starts_with("exit") => {
            return CommandResult::Failed;
            // return ExitCode::from(0);
        }
        cmd if cmd.starts_with("echo") => echo::echo(cmd),
        cmd if cmd.starts_with("type") => execute_type(cmd),
        cmd if cmd.starts_with("pwd") => pwd(),
        cmd if cmd.starts_with("cd") => cd::cd(cmd),
        cmd if cmd.starts_with("cat") => cat::cat(cmd),
        cmd if cmd.starts_with("ls") => ls::ls(cmd),
        cmd if cmd.starts_with("wc") => wc::wc(cmd),
        cmd if cmd.starts_with("tail") => tail::tail(cmd),
        _ => executor::execute(&command),
    };
}

pub fn execute_type(command: &str) -> CommandResult {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    let output: String;

    if BUILTINS.contains(&cmd) {
        output = format!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        output = format!("{} is {}", cmd, path);
    } else {
        output = format!("{}: not found", cmd.trim());
    }

    println!("{}", output);
    return CommandResult::Success;
}
