use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::commands::head::heads_or_tails;
use crate::commands::{Command, CommandResult};

pub struct Tail;
impl Command for Tail {
    fn name(&self) -> &'static str {
        "tail"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("tail ", "");
        tail(&command)
    }
}

pub fn tail(command: &str) -> CommandResult {
    if command.contains("-f") {
        let command = command.replace("-f ", "");
        return tail_watch(&command);
    }

    heads_or_tails(command, true)
}

fn tail_watch(command: &str) -> CommandResult {
    let mut ln = 0;
    let command = command.trim().to_owned();

    if let Ok(file) = File::open(command.trim()) {
        let mut reader = BufReader::new(file);

        loop {
            let mut line = String::new();
            if reader.read_line(&mut line).unwrap_or(0) == 0 {
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            let line = line.trim_end();
            println!("{}", line);

            ln += 1;
            if ln == 5 {
                break;
            }
        }
    }

    CommandResult::Success
}
