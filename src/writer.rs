use std::fs::File;
use std::io::prelude::*;
use std::io::{self, LineWriter};
use std::path::PathBuf;

pub fn write(file_name: PathBuf, contents: Vec<String>) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut file = LineWriter::new(file);

    for x in contents {
        let buff = format!("{}\n", x);
        file.write_all(buff.as_bytes())?;
    }
    Ok(())
}

pub fn make_dir(file_name: PathBuf) {
    let _ = File::create(file_name);
}
