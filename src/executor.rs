use std::{
    path::{self, PathBuf},
    process::Command,
};

use crate::{
    actions::{
        appends::{append_stderr, append_stdout},
        redirect::{redirect_stderr, redirect_stdout},
    },
    commands::CommandResult,
    enums::actions::Action,
    utils::parser,
};

pub fn execute_with_redirect<R, D>(command: &str, executor: R, default: D) -> CommandResult
where
    R: Fn(&PathBuf, &String, Vec<String>, &Action),
    D: Fn(&str) -> CommandResult,
{
    match command {
        _append_output if command.contains("2>>") => {
            append_stderr(command, executor);
            CommandResult::Success
        }
        _append_error if command.contains("1>>") || command.contains(">>") => {
            append_stdout(command, executor);
            CommandResult::Success
        }
        _redirect_error if command.contains("2>") => {
            redirect_stderr(command, executor);
            CommandResult::Success
        }
        _redirect_output if command.contains("1>") || command.contains(">") => {
            redirect_stdout(command, executor);
            CommandResult::Success
        }
        _ => default(command),
    }
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
    let args = parser::get_formatted_args(split_command[0]);
    let path = path::Path::new(split_command[1].trim()).to_path_buf();
    (path, args)
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

    (command_wo_args, args)
}

pub fn execute(command: &str) -> CommandResult {
    let (exe, args) = parser::parse_execute_command(command);

    let input = match Command::new(exe.trim()).args(&args).output() {
        Ok(output) => output,
        Err(_) => {
            println!("{}: command not found", command.trim());
            return CommandResult::Failed;
        }
    };

    let output = String::from_utf8_lossy(&input.stdout);

    print!("{}", output);
    CommandResult::Success
}
