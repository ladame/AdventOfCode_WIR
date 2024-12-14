use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_reports(file_path: &str) -> Result<i32>{
    let file: File = File::open(file_path).expect("Failed to open file");
    let reader: BufReader<File> = BufReader::new(file);
    let mut valid_reports: i32 = 0;
    for line in reader.lines(){
        let report: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        if check_report(&report){
            valid_reports += 1;
        }
    }
    Ok(valid_reports)
}
pub fn check_report(line: &[i32]) -> bool{
    let initial_index: usize = 0;
    let max_diff: i32 = 3;

    
    if line[initial_index] > line[initial_index+1]{
        return check_decrease(&line, max_diff);
    }else if line[initial_index] < line[initial_index+1]{
        return check_increase(&line, max_diff);
    }else{
        return false;
    }
}

pub fn check_decrease(line: &[i32], max_diff: i32) -> bool {
    for i in 0..(line.len()-1){
        if line[i] - line[i+1] > max_diff || line[i] - line[i+1] <= 0{
            return false;
        }
    }
    return true
}

pub fn check_increase(line: &[i32], max_diff: i32) -> bool{
    for i in (1..line.len()).rev(){
        if line[i] - line[i-1] > max_diff || line[i] - line[i-1] <= 0{
            return false;
        }
    }
    return true
}