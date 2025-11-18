use std::{fs, path};

use crate::{change_directory, writer};

pub fn ls(command: String) {
    let command_split: Vec<&str> = command.split_whitespace().collect();
    if command_split.len() > 3 {
        if command_split[3] == ">" || command_split[3] == "1>" {
            let input_path = command_split[2];
            let output_path = path::Path::new(command_split[4]);
            let entries = fs::read_dir(input_path).expect("Cannot read directory");

            let mut lines = Vec::new();
            for entry in entries {
                let entry = entry.unwrap();
                let file_name = entry.file_name().to_string_lossy().to_string();
                lines.push(file_name);
            }

            lines.sort();
            let _ = writer::write(output_path.to_path_buf(), lines);
        }
    } else {
        let current_dir = change_directory::get_curr_directory();
        for x in current_dir.iter() {
            println!("{}", x.display());
        }
    }
}
