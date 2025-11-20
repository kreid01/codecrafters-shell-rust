use std::{fs, path::PathBuf};

use crate::{executor::execute_with_redirect, parser, redirect::Action, utils::writer};

pub fn cat(command: &str) {
    let command_wo_cat = str::replace(&command, "cat ", "");
    execute_with_redirect(&command_wo_cat, execute_cat, default_cat);
}

pub fn execute_cat(output_path: &PathBuf, command: &String, _args: Vec<String>, action: &Action) {
    let err = format!("cat: {}: No such file or directory", command);

    match get_cat_result(command) {
        Ok(content) => match action {
            Action::Stdout => {
                let _ = writer::write(output_path.to_owned(), vec![content]);
            }
            Action::Stderr => {
                println!("{}", content);
            }
            Action::Append => {
                let _ = writer::append(output_path.to_owned(), vec![content]);
            }
        },
        Err(_) => match action {
            Action::Stdout => {
                println!("{}", err)
            }
            Action::Stderr => {
                let _ = writer::write(output_path.to_owned(), vec![err]);
            }
            Action::Append => {
                let _ = writer::append(output_path.to_owned(), vec![err]);
            }
        },
    }
}

pub fn default_cat(command: &str) {
    let args = parser::get_formatted_args(&command);
    let mut output = String::new();

    for arg in args {
        match get_cat_result(&arg) {
            Ok(content) => {
                output.push_str(&content.to_string());
            }
            Err(err) => {
                println!("{}", err)
            }
        }
    }

    println!("{}", output);
}

pub fn get_cat_result(command: &String) -> Result<String, String> {
    match fs::read_to_string(command) {
        Ok(contents) => Ok(contents.trim().to_string()),
        Err(_) => {
            let err = format!("cat: {}: No such file or directory", command);
            return Err(err);
        }
    }
}
