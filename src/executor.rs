use std::{path::PathBuf, process::Command};

use crate::{
    parser,
    redirect::{self},
};

pub enum Redirect {
    Stderr,
    Stdout,
}

pub fn execute_with_redirect<R, D>(command: &str, redirect: R, default: D)
where
    R: Fn(&PathBuf, &String, Vec<String>, Redirect),
    D: Fn(&str) -> Option<()>,
{
    match command {
        _redirect_error if command.contains("2>") => {
            redirect::redirect_stderr(&command, redirect, default);
        }
        _redirect_output if command.contains("1>") || command.contains(">") => {
            redirect::redirect_stdout(&command, redirect);
        }
        _ => {
            default(command);
        }
    }
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
