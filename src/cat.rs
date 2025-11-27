use std::{fs, path::PathBuf};

use crate::{
    enums::actions::Action,
    executor::execute_with_redirect,
    utils::{
        parser,
        writer::{self, make_file},
    },
    Command,
};

pub enum CommandResult {
    Output(String),
    Success,
    Failed,
}

pub struct Cat;
impl Command for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = str::replace(&cmd, "cat ", "");
        return cat(&command);
    }
}

pub fn cat(command: &str) -> CommandResult {
    return execute_with_redirect(&command, execute_cat, default_cat);
}

pub fn execute_cat(output_path: &PathBuf, command: &String, _args: Vec<String>, action: &Action) {
    let err = format!("cat: {}: No such file or directory", command);

    match get_cat_result(command) {
        Ok(content) => match action {
            Action::RedirectStdout => {
                let _ = writer::write(output_path.to_owned(), vec![content]);
            }
            Action::RedirectStderr => {
                println!("{}", content.trim());
                make_file(output_path.to_owned())
            }
            Action::AppendStdout => {
                let _ = writer::append(output_path.to_owned(), vec![content]);
            }
            Action::AppendStderr => {
                println!("{}", content.trim());
                make_file(output_path.to_owned())
            }
        },
        Err(_) => match action {
            Action::RedirectStdout => {
                println!("{}", err);
                make_file(output_path.to_owned())
            }
            Action::RedirectStderr => {
                let _ = writer::write(output_path.to_owned(), vec![err]);
            }
            Action::AppendStdout => {
                println!("{}", err);
                make_file(output_path.to_owned())
            }
            Action::AppendStderr => {
                let _ = writer::append(output_path.to_owned(), vec![err]);
            }
        },
    }
}

pub fn default_cat(command: &str) -> CommandResult {
    let args = parser::get_formatted_args(&command);
    let mut output = String::new();

    for arg in args {
        match get_cat_result(&arg) {
            Ok(content) => {
                output.push_str(&content.to_string());
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    return CommandResult::Output(output);
}

pub fn get_cat_result(command: &String) -> Result<String, String> {
    match fs::read_to_string(command) {
        Ok(contents) => Ok(contents.to_string()),
        Err(_) => {
            let err = format!("cat: {}: No such file or directory", command);
            return Err(err);
        }
    }
}
