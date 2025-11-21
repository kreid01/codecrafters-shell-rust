use std::collections::HashSet;

use crate::{exe, BUILTINS};

pub fn get_autocomplete_options(command: &str) -> Vec<String> {
    let mut options = get_options();
    let mut output: Vec<String> = Vec::new();
    options.sort();

    for option in options.iter().filter(|x| x.starts_with(command)) {
        output.push(option.to_string());
    }

    return output;
}

pub fn get_autocomplete_prefix(options: &Vec<String>) -> String {
    let mut prefix = options[0].clone();

    for s in options.iter() {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return String::new();
            }
            prefix.pop();
        }
    }

    prefix
}

pub fn autocomplete(command: &String) -> String {
    let options = get_options();
    for option in options {
        if option.starts_with(command) {
            return format!("{} ", option).to_string().to_owned();
        }
    }

    return format!("{}\x07", command.to_owned());
}

fn get_options() -> Vec<String> {
    let mut builtins = BUILTINS.map(|x| x.to_string()).to_vec();
    let mut exes = exe::get_exe_paths();

    exes.append(&mut builtins);
    return exes
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<String>>();
}
