use std::collections::VecDeque;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::commands::{get_commands, history, CommandResult};
use crate::utils::input_handler::{handle_input, InputResult};

mod actions;
mod commands;
mod enums;
mod executor;
mod utils;

const BUILTINS: [&str; 6] = ["exit", "echo", "type", "pwd", "cd", "history"];

fn main() -> ExitCode {
    let mut history: Vec<String> = Vec::new();
    let mut appended_history: Vec<String> = Vec::new();

    loop {
        print!("\r$ ");
        io::stdout().flush().unwrap();

        let buffer = match handle_input(&history) {
            InputResult::Completed(input) => input,
            InputResult::Exit(code) => return code,
        };

        if buffer.is_empty() {
            continue;
        }

        history.push(buffer.to_owned());

        let mut commands_queue: VecDeque<&str> = buffer.split("|").collect();
        let commands = get_commands();

        let mut piped_args = String::new();

        while let Some(cmd) = commands_queue.pop_front() {
            let cmd = cmd.trim();
            if cmd.starts_with("exit") {
                return ExitCode::from(0);
            }

            if cmd.starts_with("history -r") {
                let mut file_history = history::read_file_history(cmd);
                history.append(&mut file_history);
                break;
            }

            if cmd.starts_with("history -w") {
                history::write_file_history(cmd, &history);
                break;
            }

            if cmd.starts_with("history -a") {
                let history_to_append = history
                    .iter()
                    .filter(|x| !appended_history.contains(x))
                    .map(|x| x.to_string())
                    .collect();
                history::append_file_history(cmd, &history_to_append);
                appended_history = history.to_owned();
                break;
            }

            if cmd.starts_with("history") {
                history::history(cmd, &history);
                break;
            }

            let mut handled = false;

            for command in &commands {
                let args = format!("{} {}", cmd, piped_args);
                if cmd.starts_with(command.name()) {
                    if let CommandResult::Output(output) = command.run(&args) {
                        if commands_queue.is_empty() {
                            println!("{}", output.trim())
                        } else {
                            piped_args = output;
                        }
                    }
                    handled = true;
                    break;
                }
            }

            if !handled {
                let args = format!("{} {}", cmd, piped_args);
                executor::execute(&args);
            }
        }
    }
}
