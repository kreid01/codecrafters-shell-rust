use crate::executor::BuiltInCommand;
use std::path;

use crate::{executor::execute_with_redirect, parser, writer};

pub fn echo(command: String) {
    let command_wo_echo = str::replace(&command, "echo ", "");
    execute_with_redirect(
        &command_wo_echo,
        write_echo,
        default_echo,
        BuiltInCommand::Echo,
    );
}

pub fn write_echo(output_path: path::PathBuf, args: Vec<String>) {
    let mut lines = Vec::new();

    for arg in args {
        lines.push(arg);
    }

    let _ = writer::write(output_path, lines);
}

pub fn default_echo(command: &str) {
    let formatted_command = parser::format_string_command(&command);
    println!("{}", &formatted_command.trim())
}
