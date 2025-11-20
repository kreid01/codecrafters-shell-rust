use std::{fs, path::PathBuf};

use crate::{
    executor::{execute_with_redirect, Redirect},
    parser, writer,
};

pub fn cat(command: &str) {
    let command_wo_cat = str::replace(&command, "cat ", "");
    execute_with_redirect(&command_wo_cat, write_cat, default_cat);
}

pub fn default_cat(command: &str) -> Option<()> {
    let args = parser::get_formatted_args(&command);
    let mut output = String::new();

    for arg in args {
        match get_cat(&arg) {
            Ok(content) => {
                output.push_str(&content.to_string());
            }
            Err(_) => {
                return None;
            }
        }
    }

    println!("{}", output);
    return Some(());
}

pub fn write_cat(output_path: &PathBuf, command: &String, _args: Vec<String>, redirect: Redirect) {
    let err = format!("cat: {}: No such file or directory", command);

    match get_cat(command) {
        Ok(content) => match redirect {
            Redirect::Stdout => {
                let _ = writer::write(output_path.to_owned(), vec![content]);
            }
            Redirect::Stderr => {
                println!("{}", content);
            }
        },
        Err(_) => match redirect {
            Redirect::Stdout => {
                println!("{}", err)
            }
            Redirect::Stderr => {
                let _ = writer::write(output_path.to_owned(), vec![err]);
            }
        },
    }
}

pub fn get_cat(command: &String) -> Result<String, &str> {
    match fs::read_to_string(command) {
        Ok(contents) => Ok(contents.trim().to_string()),
        Err(_) => Err("cat: {}: No such file or directory"),
    }
}
