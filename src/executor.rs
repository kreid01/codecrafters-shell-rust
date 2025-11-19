use std::{path::PathBuf, process::Command};

use crate::{
    parser,
    redirect::{self, error_default},
};

#[derive(PartialEq)]
pub enum BuiltInCommand {
    Cat,
    Echo,
}

impl BuiltInCommand {
    fn as_str(&self) -> &str {
        match self {
            BuiltInCommand::Cat => "cat",
            BuiltInCommand::Echo => "echo",
            // BuiltInCommand::Ls => "ls",
        }
    }
}

pub fn execute_with_redirect<R, D>(command: &str, redirect: R, default: D, built_in: BuiltInCommand)
where
    R: Fn(PathBuf, Vec<String>),
    D: Fn(&str),
{
    match command {
        _no_error if built_in == BuiltInCommand::Echo => {
            error_default(command, default);
        }
        _redirect_error if command.contains("2>") => {
            redirect::redirect_stderr(&command, redirect);
        }
        _redirect_output if command.contains("1>") => {
            redirect::redirect_stdout(&command, redirect);
        }
        _ => default(command),
    }
}

pub fn execute_command(command: BuiltInCommand, args: Vec<String>) -> Option<String> {
    let input = match Command::new(command.as_str()).args(&args).output() {
        Ok(output) => output,
        Err(_) => {
            println!("{}: {}: Command not found", command.as_str(), args[0]);
            return None;
        }
    };

    return Some(String::from_utf8_lossy(&input.stdout).trim().to_string());
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
