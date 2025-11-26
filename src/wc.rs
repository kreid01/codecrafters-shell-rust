use std::{fs::File, io::Read};

use crate::cat::CommandResult;

pub fn wc(args: &str) -> CommandResult {
    println!("{}", args);
    let file = File::open(args);
    let contents: String = match file {
        Ok(mut f) => {
            let mut contents = String::new();
            let _ = f.read_to_string(&mut contents);
            contents
        }
        Err(_) => args.to_string(),
    };

    let result = format!(
        " {:>7} {:>7} {:>7}",
        contents.lines().count(),
        get_word_count(&contents),
        contents.len()
    );

    println!("{}", result);
    return CommandResult::Success;
}

pub fn get_word_count(string: &String) -> usize {
    let words_spaced = string.replace("\n", " ");
    return words_spaced.split_whitespace().count();
}
