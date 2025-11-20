use regex::Regex;
use std::path::PathBuf;

use crate::{enums::actions::Action, executor::execute_commands_with_args};

pub fn redirect_stderr<F>(command: &str, executor: F)
where
    F: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let commands: Vec<&str> = command.split("2>").collect();
    execute_commands_with_args(commands, executor, Action::RedirectStderr);
}

pub fn redirect_stdout<F>(command: &str, executor: F)
where
    F: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let re = Regex::new(r"1>|>").unwrap();
    let commands: Vec<&str> = re.split(command).collect();
    execute_commands_with_args(commands, executor, Action::RedirectStdout);
}
