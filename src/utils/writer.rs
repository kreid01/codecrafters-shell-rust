use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, LineWriter};
use std::path::PathBuf;

pub fn write(file_name: PathBuf, contents: Vec<String>) -> io::Result<()> {
    let file = File::create(file_name)?;
    let mut file = LineWriter::new(file);

    for x in contents {
        let buff = format!("\r{}\n", x);
        file.write_all(buff.as_bytes())?;
    }
    Ok(())
}

pub fn append(file_name: PathBuf, contents: Vec<String>) -> io::Result<()> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)?;

    let mut writer = LineWriter::new(file);

    for line in contents {
        writeln!(writer, "\r{}", line)?;
    }

    Ok(())
}

pub fn make_file(file_name: PathBuf) {
    let _ = OpenOptions::new().append(true).create(true).open(file_name);
}
