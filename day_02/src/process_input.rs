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

pub fn check_report(line: &Vec<i32>) -> bool{
    let max_diff: i32 = 3;

    // Check increase or decrease
    if check_consistency(line, max_diff){
        return true;
    }
    // if not consistent, check for errors
    return report_correction(line, max_diff)
    
}

pub fn check_consistency(line: &Vec<i32>, max_diff: i32) -> bool{
    let mut decrease: bool = true;
    let mut increase: bool = true;

    for i in 0..(line.len()-1){
        if line[i] == line[i+1] || (line[i] - line[i+1]).abs() > max_diff{
            return false;
        }
        if line[i] < line[i+1]{
            decrease = false;
        }else{
            increase = false;
        }
    }

    decrease ||  increase
    
}

pub fn report_correction(line: &Vec<i32>, max_diff: i32) -> bool{
    let mut copied_line: Vec<i32> = line.clone();
    for i in 0..line.len(){
        copied_line.remove(i);
        if check_consistency(&copied_line, max_diff){
            return true;
        }
        copied_line.insert(i, line[i]);
    }
    return false
}