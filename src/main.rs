use std::env::{self};
use std::fs;
use std::io::{self, stdin, stdout, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::ExitCode;
use std::result::Result::Ok;
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod executor;
mod ls;
mod utils;

const BUILTINS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

fn main() -> ExitCode {
    loop {
        print!("\r$ ");
        io::stdout().flush().unwrap();

        let mut stdout = stdout().into_raw_mode().unwrap();
        let stdin = stdin();

        let mut command = String::new();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => {
                    println!("\r");
                    break;
                }
                Key::Char('\t') => {
                    let autocomplete = autocomplete(&command);
                    print!("{}\r$ {}", clear::CurrentLine, autocomplete);
                    command = autocomplete;
                }
                Key::Backspace => {
                    command.pop();
                    write!(stdout, "\x08 \x08").unwrap();
                }
                Key::Char(c) => {
                    command.push(c);
                    write!(stdout, "{}", c).unwrap();
                }
                Key::Ctrl('c') => {
                    return ExitCode::from(0);
                }
                _ => {}
            }

            stdout.flush().unwrap();
        }

        if command.is_empty() {
            continue;
        }

        match command {
            cmd if cmd.starts_with("exit") => {
                return ExitCode::from(0);
            }
            cmd if cmd.starts_with("echo") => echo::echo(cmd),
            cmd if cmd.starts_with("type") => execute_type(cmd),
            cmd if cmd.starts_with("pwd") => pwd(),
            cmd if cmd.starts_with("cd") => cd::cd(cmd.as_str()),
            cmd if cmd.starts_with("cat") => cat::cat(cmd.as_str()),
            cmd if cmd.starts_with("ls") => ls::ls(cmd),
            _ => executor::execute(&command),
        }
    }
}

pub fn autocomplete(command: &String) -> String {
    for x in BUILTINS {
        if x.starts_with(command) {
            return format!("{} ", x).to_string().to_owned();
        }
    }

    let exes = get_exe_paths();
    for x in exes {
        if x.starts_with(command) {
            return format!("{} ", x).to_string().to_owned();
        }
    }

    return format!("{}\x07", command.to_owned());
}

fn pwd() {
    let curr_dir = env::current_dir().unwrap();
    println!("{}", curr_dir.display())
}

pub fn execute_type(command: String) {
    let cmd = command.split_whitespace().nth(1).unwrap_or("");
    if BUILTINS.contains(&cmd) {
        println!("{} is a shell builtin", cmd);
    } else if let Some(path) = get_exe_path(cmd) {
        println!("{} is {}", cmd, path);
    } else {
        println!("{}: not found", cmd.trim());
    }
}

fn get_exe_paths() -> Vec<String> {
    let mut exes: Vec<String> = Vec::new();
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            match fs::read_dir(dir) {
                Ok(entries) => {
                    for entry in entries {
                        let entry = entry.unwrap();
                        if let Ok(metadata) = entry.metadata() {
                            let permissions = metadata.permissions();
                            if permissions.mode() & 0o111 != 0 {
                                exes.push(entry.file_name().to_string_lossy().to_string());
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }

    return exes;
}

fn get_exe_path(command: &str) -> Option<String> {
    if let Ok(paths) = env::var("PATH") {
        for dir in env::split_paths(&paths) {
            let path = dir.join(command);
            if path.is_file() && is_exe(&path) {
                Some(path.to_string_lossy().to_string());
            }
        }
    }

    None
}

pub fn is_exe(path: &PathBuf) -> bool {
    if let Ok(metadata) = path.metadata() {
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 != 0 {
            return true;
        }
    }

    return false;
}
