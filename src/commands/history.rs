pub fn history(cmd: &str, history: &[String]) {
    let count = cmd.replace("history ", "").parse::<usize>().unwrap_or(10);
    let diff = history.len() - count;

    for (i, command) in history.iter().enumerate().skip(diff) {
        println!("\t{} {}", i + 1, command)
    }
}
