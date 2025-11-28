use std::fs::File;
use std::io::Read;

use regex::Regex;

use crate::commands::{Command, CommandResult};
use crate::utils::parser::get_formatted_args;

pub struct Head;
impl Command for Head {
    fn name(&self) -> &'static str {
        "head"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("head ", "");
        head(&command)
    }
}

pub fn head(command: &str) -> CommandResult {
    heads_or_tails(command, true)
}

pub fn heads_or_tails(command: &str, tails: bool) -> CommandResult {
    let args = get_formatted_args(command);
    let mut contents: Vec<String> = Vec::new();
    let count = get_count(&args);

    let command_wo_args = command.replace("-n ", "");
    let re = Regex::new(r"\d+ ").unwrap();
    let cleaned_args = re.replace(&command_wo_args, "").to_string();

    match File::open(&cleaned_args) {
        Ok(mut file) => {
            let mut text = String::new();
            file.read_to_string(&mut text).expect("reading file");

            if tails {
                for line in text.lines().rev().take(count) {
                    contents.push(line.to_string());
                }
            } else {
                for line in text.lines().take(count) {
                    contents.push(line.to_string());
                }
            }
        }
        Err(_) => {
            if tails {
                for line in cleaned_args.lines().rev().take(count) {
                    contents.push(line.to_string());
                }
            } else {
                for line in cleaned_args.lines().take(count) {
                    contents.push(line.to_string());
                }
            }
        }
    };

    CommandResult::Output(format!("{}\n", contents.join("\n")))
}

fn get_count(args: &[String]) -> usize {
    if args.first().unwrap() == "-n" {
        let count = args.get(1).unwrap();
        return count.parse::<usize>().unwrap_or(10);
    }

    10
}
