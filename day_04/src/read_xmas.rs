use crate::xmas::Xmas;
use std::io;

pub fn read_xmas(file_path: &str) -> Result<i32, io::Error> {
    let words= "XMAS";
    let mut xmas: Xmas = Xmas::new(file_path);
    xmas.find_words(&words);
    Ok(xmas.get_total_xmas())
}
