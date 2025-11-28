pub fn history(history: &Vec<String>) {
    for (i, command) in history.iter().enumerate() {
        println!("\t{} {}", i + 1, command)
    }
}
