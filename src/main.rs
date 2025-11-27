use std::collections::VecDeque;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::commands::{get_commands, CommandResult};
use crate::utils::input_handler::{handle_input, InputResult};

mod actions;
mod commands;
mod enums;
mod executor;
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

        let mut commands_queue: VecDeque<&str> = buffer.split("|").collect();
        let commands = get_commands();

        let mut piped_args = String::new();

        while let Some(cmd) = commands_queue.pop_front() {
            let cmd = cmd.trim();
            if cmd.starts_with("exit") {
                return ExitCode::from(0);
            }

            let mut handled = false;

            for command in &commands {
                if cmd.starts_with(command.name()) {
                    let args = format!("{} {}", cmd, piped_args);
                    match command.run(&args) {
                        CommandResult::Output(output) => {
                            if commands_queue.is_empty() {
                                println!("{}", output.trim())
                            } else {
                                piped_args = output;
                            }
                        }
                        _ => {}
                    }
                    handled = true;
                    break;
                }
            }

            if !handled {
                executor::execute(&cmd);
            }
        }
    }
}
