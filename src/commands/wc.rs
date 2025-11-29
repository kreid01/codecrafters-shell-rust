use std::{fs::File, io::Read};

use crate::commands::{Command, CommandResult};

pub struct Wc;
impl Command for Wc {
    fn name(&self) -> &'static str {
        "wc"
    }
    fn run(&self, cmd: &str) -> CommandResult {
        let command = cmd.replace("wc ", "");
        wc(&command)
    }
}

pub fn wc(command: &str) -> CommandResult {
    let file = File::open(command);
    let contents: String = match file {
        Ok(mut f) => {
            let mut contents = String::new();
            let _ = f.read_to_string(&mut contents);
            contents
        }
        Err(_) => command.to_string(),
    };

    let result = format!(
        " {:>7} {:>7} {:>7}",
        contents.lines().count(),
        get_word_count(&contents),
        contents.len()
    );

    println!("{}", result);
    CommandResult::Success
}

pub fn get_word_count(string: &str) -> usize {
    let words_spaced = string.replace("\n", " ");
    words_spaced.split_whitespace().count()
}
