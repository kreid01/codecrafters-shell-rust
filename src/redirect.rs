use regex::Regex;
use std::path::{self, PathBuf};

use crate::{executor::Redirect, parser, writer::make_dir};

pub fn redirect_stderr<F, D>(command: &str, executor: F, default: D)
where
    F: Fn(&PathBuf, &String, Vec<String>, Redirect),
    D: Fn(&str) -> Option<()>,
{
    let command_split: Vec<&str> = command.split("2>").collect();
    let (output_path, commands): (PathBuf, Vec<String>) = get_redirect_args(command_split);
    let (commands, args) = get_commands_and_args(commands);

    for command in commands {
        executor(&output_path, &command, args.clone(), Redirect::Stderr);
    }
}

pub fn redirect_stdout<F>(command: &str, executor: F)
where
    F: Fn(&PathBuf, &String, Vec<String>, Redirect),
{
    let re = Regex::new(r"1>|>").unwrap();
    let command_split: Vec<&str> = re.split(command).collect();

    let (output_path, commands) = get_redirect_args(command_split);
    let (commands, args) = get_commands_and_args(commands);

    for command in commands {
        executor(&output_path, &command, args.clone(), Redirect::Stdout);
    }
}

pub fn get_redirect_args(split_command: Vec<&str>) -> (PathBuf, Vec<String>) {
    let args = parser::get_formatted_args(&split_command[0]);
    let path = path::Path::new(split_command[1].trim()).to_path_buf();
    return (path, args);
}

pub fn get_commands_and_args(commands: Vec<String>) -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = commands
        .iter()
        .filter(|x| x.starts_with('-'))
        .cloned()
        .collect();

    let command_wo_args: Vec<String> = commands
        .iter()
        .filter(|x| !x.starts_with('-'))
        .cloned()
        .collect();

    return (command_wo_args, args);
}
