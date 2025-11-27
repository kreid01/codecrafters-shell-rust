use std::path;

use crate::{
    commands::{Command, CommandResult},
    enums::actions::Action,
    executor::execute_with_redirect,
    utils::{
        parser, printer,
        writer::{self, make_file},
    },
};

pub struct Echo;
impl Command for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("echo ", "");
        echo(&command)
    }
}

pub fn echo(command: &str) -> CommandResult {
    return execute_with_redirect(&command, write_echo, default_echo);
}

pub fn write_echo(
    output_path: &path::PathBuf,
    command: &String,
    _args: Vec<String>,
    action: &Action,
) {
    let mut lines = Vec::new();
    lines.push(command.to_owned());

    match action {
        Action::RedirectStdout => {
            let _ = writer::write(output_path.to_owned(), lines);
        }
        Action::RedirectStderr => {
            echo_stderr(output_path, lines);
        }
        Action::AppendStdout => {
            let _ = writer::append(output_path.to_owned(), lines);
        }
        Action::AppendStderr => {
            echo_stderr(output_path, lines);
        }
    }
}

pub fn echo_stderr(output_path: &path::PathBuf, lines: Vec<String>) {
    printer::print_lines(lines);
    make_file(output_path.to_owned());
}

pub fn default_echo(command: &str) -> CommandResult {
    let formatted_command = parser::format_string_command(&command);
    return CommandResult::Output(formatted_command.to_string());
}
