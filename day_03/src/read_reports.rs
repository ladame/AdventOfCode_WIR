use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use crate::scan_reports::scan_line;

pub fn read_reports(file_path: &str) -> Result<i32> {
    let file: File = File::open(file_path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);
    let mut total: i32 = 0;
    for line in reader.lines(){
        scan_line(&line?, &mut total);
    }
    return Ok(total)
}