use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::cat::CommandResult;
use crate::Command;

pub struct Tail;
impl Command for Tail {
    fn name(&self) -> &'static str {
        "tail"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("tail -f ", "");
        return tail(&command);
    }
}

pub fn tail(command: &str) -> CommandResult {
    let path = command.to_owned();
    let piped_buffer: Vec<&str> = command.split('|').map(|x| x.trim()).collect();
    let next_command = piped_buffer.last().unwrap();

    let ln = 0;

    match next_command {
        cmd if cmd.starts_with("head -n") => {}
        _ => {}
    }

    let fp = Arc::new(Mutex::new(path));
    let lnp = Arc::new(Mutex::new(ln));

    thread::spawn(move || {
        match fp.lock() {
            Ok(fp_unlocked) => {
                if let Ok(file) = File::open(&*fp_unlocked.trim()) {
                    let mut reader = BufReader::new(file);

                    loop {
                        if let Ok(unlocked_lnp) = lnp.lock() {
                            if *unlocked_lnp == 5 {
                                break;
                            }
                        }

                        let mut line = String::new();

                        if reader.read_line(&mut line).unwrap_or(0) == 0 {
                            thread::sleep(Duration::from_millis(100));
                        } else {
                            print!("\r{}", line);
                            match lnp.lock() {
                                Ok(mut lnp_l) => *lnp_l += 1,
                                Err(_) => {}
                            }
                        }
                    }
                }
            }
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {}", poisoned);
            }
        };
    });

    return CommandResult::Success;
}
