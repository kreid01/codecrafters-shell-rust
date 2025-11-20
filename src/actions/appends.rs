use std::path::PathBuf;

use regex::Regex;

use crate::{enums::actions::Action, executor::execute_commands_with_args};

pub fn append_stdout<R>(command: &str, executor: R)
where
    R: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let re = Regex::new(r"1>>|>>").unwrap();
    let commands: Vec<&str> = re.split(command).collect();
    execute_commands_with_args(commands, executor, Action::AppendStdout);
}

pub fn append_stderr<R>(command: &str, executor: R)
where
    R: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let commands: Vec<&str> = command.split("2>>").collect();
    execute_commands_with_args(commands, executor, Action::AppendStderr);
}
