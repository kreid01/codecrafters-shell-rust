pub fn history(cmd: &str, history: &[String]) {
    let count = cmd.replace("history ", "").parse::<usize>().unwrap_or(10);

    for (i, command) in history.iter().enumerate().take(count) {
        println!("\t{} {}", i + 1, command)
    }
}
