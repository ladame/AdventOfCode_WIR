use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_xmas(file_path: &str) -> Result<i32>{
    let file: File = File::open(file_path)?;
    let contents: BufReader<File> = BufReader::new(file);
    for line in contents.lines(){

    }
    Ok(1000)
}