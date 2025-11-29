use crate::commands::cat::get_cat_result;

pub fn history(cmd: &str, history: &[String]) {
    let diff = match cmd.replace("history ", "").parse::<usize>() {
        Ok(count) => history.len() - count,
        Err(_) => 0,
    };

    for (i, command) in history.iter().enumerate().skip(diff) {
        println!("\t{} {}", i + 1, command)
    }
}

pub fn history_file(cmd: &str) -> Vec<String> {
    let cmd = cmd.replace("history -r ", "");
    let mut file_history = String::new();

    if let Ok(file) = get_cat_result(&cmd) {
        file_history = file;
    }

    file_history.split("\n").map(|s| s.to_string()).collect()
}
