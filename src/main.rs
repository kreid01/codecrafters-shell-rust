use std::collections::VecDeque;
use std::io::{self, Write};
use std::process::ExitCode;

use crate::cat::{Cat, CommandResult};
use crate::cd::Cd;
use crate::echo::Echo;
use crate::execute_type::Type;
use crate::ls::Ls;
use crate::pwd::Pwd;
use crate::tail::Tail;
use crate::utils::input_handler::{handle_input, InputResult};
use crate::wc::Wc;

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod exe;
mod execute_type;
mod executor;
mod ls;
mod pwd;
mod tail;
mod utils;
mod wc;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

trait Command {
    fn name(&self) -> &'static str;
    fn run(&self, cmd: &str) -> CommandResult;
}

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
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(Echo),
            Box::new(Pwd),
            Box::new(Cd),
            Box::new(Cat),
            Box::new(Type),
            Box::new(Ls),
            Box::new(Wc),
            Box::new(Tail),
        ];

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
