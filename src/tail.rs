use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::cat::CommandResult;

pub fn tail(command: &str) -> CommandResult {
    let file_path = command.replace("tail -f", "");

    let piped_buffer: Vec<&str> = command.split('|').map(|x| x.trim()).collect();
    let next_command = piped_buffer.last().unwrap();

    match next_command {
        cmd if cmd.starts_with("head") => {}
        _ => {}
    }

    let mut ln = 0;

    let fp = Arc::new(Mutex::new(file_path.clone()));
    let lnp = Arc::new(Mutex::new(ln));

    let (tx, rx) = channel();

    thread::spawn(move || {
        match fp.lock() {
            Ok(fp_unlocked) => {
                let file = File::open(&*fp_unlocked).unwrap();
                let mut reader = BufReader::new(file);

                loop {
                    match lnp.lock() {
                        Ok(lnl_l) => {
                            if *lnl_l == 5 {
                                break;
                            }
                        }
                        Err(_) => {}
                    }

                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        thread::sleep(Duration::from_millis(100));
                    } else {
                        print!("{}", line);
                        tx.send(line).unwrap();
                        match lnp.lock() {
                            Ok(mut lnp_l) => *lnp_l += 1,
                            Err(_) => {}
                        }
                    }
                }
            }
            Err(poisoned) => {
                eprintln!("Mutex poisoned: {}", poisoned);
            }
        };
    });

    let mut lines = String::new();
    for received in rx.iter() {
        lines.push_str(&received);
    }

    return CommandResult::Output(lines);
}
