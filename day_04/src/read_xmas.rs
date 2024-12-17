use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_xmas(file_path: &str) -> Result<i32>{
    let file: File = File::open(file_path)?;
    let contents: BufReader<File> = BufReader::new(file);
    for line in contents.lines(){
        // Check the matrix size of the input file
        // Find word Xmas horizontally
        // Find word Xmas horizontally backwards
        // Find words Xmas vertically
        // Find words Xmas vertically backwards
        // Find words Xmas diagonally
        // Find words Xmas diagonally backwards
    }
    Ok(1000)
}