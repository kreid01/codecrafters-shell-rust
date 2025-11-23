use std::io::{self, stdin, stdout, Write};
use std::process::ExitCode;

use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::cd::pwd;
use crate::exe::get_exe_path;
use crate::utils::autocomplete::{self, get_autocomplete_options, get_autocomplete_prefix};

mod actions;
mod cat;
mod cd;
mod echo;
mod enums;
mod exe;
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

        let mut buffer = String::new();
        let mut last_key: Option<Key> = None;

        for c in stdin.keys().flatten() {
            match c {
                Key::Char('\n') => {
                    stdout.suspend_raw_mode().unwrap();
                    print!("\n");
                    stdout.activate_raw_mode().unwrap();
                    break;
                }
                Key::Char('\t') => {
                    let autocomplete_options = get_autocomplete_options(&buffer);

                    if let Some((autocomplete_prefix, count)) =
                        get_autocomplete_prefix(&autocomplete_options)
                    {
                        if autocomplete_prefix != buffer {
                            let suffix = if count == 1 { " " } else { "" };
                            print!("\r$ {}{}", autocomplete_prefix, suffix);
                            stdout.flush().unwrap();
                            buffer = format!("{}{}", autocomplete_prefix, suffix);
                            continue;
                        }
                    }
                    match autocomplete_options.len() {
                        0 => {
                            print!("\x07")
                        }
                        1 => {
                            let autocomplete = autocomplete::autocomplete(&buffer);
                            print!("{}\r$ {}", clear::CurrentLine, autocomplete);
                            buffer = autocomplete;
                        }
                        _ => {
                            if let Some(Key::Char('\t')) = last_key {
                                stdout.suspend_raw_mode().unwrap();
                                println!();
                                print!("{}\n", autocomplete_options.join("  "));
                                stdout.activate_raw_mode().unwrap();
                                print!("$ {}", buffer);
                            } else {
                                print!("\x07");
                            }
                        }
                    }
                }
                Key::Backspace => {
                    buffer.pop();
                    write!(stdout, "\x08 \x08").unwrap();
                }
                Key::Char(c) => {
                    buffer.push(c);
                    write!(stdout, "{}", c).unwrap();
                }
                Key::Ctrl('c') => {
                    return ExitCode::from(0);
                }
                _ => continue,
            }

            stdout.flush().unwrap();
            last_key = Some(c);
        }

        stdout.suspend_raw_mode().unwrap();

        if buffer.is_empty() {
            continue;
        }

        match buffer {
            cmd if cmd.starts_with("exit") => {
                return ExitCode::from(0);
            }
            cmd if cmd.starts_with("echo") => echo::echo(cmd),
            cmd if cmd.starts_with("type") => execute_type(cmd),
            cmd if cmd.starts_with("pwd") => pwd(),
            cmd if cmd.starts_with("cd") => cd::cd(cmd.as_str()),
            cmd if cmd.starts_with("cat") => cat::cat(cmd.as_str()),
            cmd if cmd.starts_with("ls") => ls::ls(cmd),
            _ => executor::execute(&buffer),
        }
    }
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
