use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::process::ExitCode;

use termion::clear;
use termion::input::TermRead;
use termion::{event::Key, raw::IntoRawMode};

use crate::utils::autocomplete;
use crate::utils::autocomplete::get_autocomplete_options;
use crate::utils::autocomplete::get_autocomplete_prefix;

pub enum InputResult {
    Completed(String),
    Exit(ExitCode),
}

pub fn handle_input() -> InputResult {
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
                return InputResult::Exit(ExitCode::from(0));
            }
            _ => continue,
        }

        stdout.flush().unwrap();
        last_key = Some(c);
    }

    stdout.suspend_raw_mode().unwrap();

    return InputResult::Completed(buffer);
}
