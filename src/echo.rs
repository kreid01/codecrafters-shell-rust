use std::path;

use crate::{
    executor::{execute_with_redirect, Redirect},
    parser,
    writer::{self, make_dir},
};

pub fn echo(command: String) {
    let command_wo_echo = str::replace(&command, "echo ", "");
    execute_with_redirect(&command_wo_echo, write_echo, default_echo);
}

pub fn write_echo(
    output_path: &path::PathBuf,
    command: &String,
    _args: Vec<String>,
    redirect: Redirect,
) {
    let mut lines = Vec::new();
    lines.push(command.to_owned());

    match redirect {
        Redirect::Stdout => {
            let _ = writer::write(output_path.to_owned(), lines);
        }
        Redirect::Stderr => {
            for line in lines {
                println!("{}", line);
            }

            make_dir(output_path.to_owned());
        }
    }
}

pub fn default_echo(command: &str) -> Option<()> {
    let formatted_command = parser::format_string_command(&command);
    println!("{}", &formatted_command.trim());
    Some(())
}
