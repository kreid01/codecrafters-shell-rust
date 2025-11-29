pub fn history(cmd: &str, history: &[String]) {
    let diff = match cmd.replace("history ", "").parse::<usize>() {
        Ok(count) => history.len() - count,
        Err(_) => 0,
    };

    for (i, command) in history.iter().enumerate().skip(diff) {
        println!("\t{} {}", i + 1, command)
    }
}
