use std::{
    default,
    path::{self, PathBuf},
    process::Command,
};

use regex::Regex;

use crate::{
    parser,
    redirect::{self, Action},
};

pub fn execute_with_redirect<R, D>(command: &str, executor: R, default: D)
where
    R: Fn(&PathBuf, &String, Vec<String>, &Action),
    D: Fn(&str),
{
    match command {
        _append_output if command.contains("1>>") || command.contains(">>") => {
            append_stderr(command, executor);
        }
        _redirect_error if command.contains("2>") => {
            redirect::redirect_stderr(&command, executor);
        }
        _redirect_output if command.contains("1>") || command.contains(">") => {
            redirect::redirect_stdout(&command, executor);
        }
        _ => {
            default(command);
        }
    }
}

pub fn append_stderr<R>(command: &str, executor: R)
where
    R: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let re = Regex::new(r"1>>|>>").unwrap();
    let commands: Vec<&str> = re.split(command).collect();
    execute_commands_with_args(commands, executor, Action::Append);
}

pub fn execute_commands_with_args<F>(commands: Vec<&str>, executor: F, action: Action)
where
    F: Fn(&PathBuf, &String, Vec<String>, &Action),
{
    let (output_path, commands): (PathBuf, Vec<String>) = get_args(commands);
    let (commands, args) = get_commands_and_args(commands);

    for command in commands {
        executor(&output_path, &command, args.clone(), &action);
    }
}

pub fn get_args(split_command: Vec<&str>) -> (PathBuf, Vec<String>) {
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

pub fn execute(command: &str) {
    let (exe, args) = parser::parse_execute_command(command);

    let input = match Command::new(exe.trim()).args(args).output() {
        Ok(output) => output,
        Err(_) => {
            println!("{}: command not found", command.trim());
            return;
        }
    };

    let output = String::from_utf8_lossy(&input.stdout);
    print!("{}", output);
}
