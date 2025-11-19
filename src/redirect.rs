use std::path::{self, PathBuf};

use crate::{parser, writer::make_dir};

pub fn redirect_stderr<F>(command: &str, executor: F)
where
    F: Fn(PathBuf, Vec<String>),
{
    let command_split: Vec<&str> = command.split("2>").collect();
    let (output_path, args) = get_redirect_args(command_split);
    executor(output_path, args);
}

pub fn redirect_stdout<F>(command: &str, executor: F)
where
    F: Fn(PathBuf, Vec<String>),
{
    let command_split: Vec<&str> = command.split("1>").collect();
    let (output_path, args) = get_redirect_args(command_split);
    executor(output_path, args);
}

pub fn error_default<D>(command: &str, default: D)
where
    D: Fn(&str),
{
    let command_split: Vec<&str> = command.split("2>").collect();
    let (output_location, args) = get_redirect_args(command_split);

    make_dir(output_location);
    default(args[0].as_str());
}

pub fn get_redirect_args(split_command: Vec<&str>) -> (PathBuf, Vec<String>) {
    let args = parser::get_formatted_args(&split_command[0]);
    let path = path::Path::new(split_command[1].trim()).to_path_buf();
    return (path, args);
}
