use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::min;



pub fn split_data(file_path: &str) -> io::Result<(Vec<i32>,Vec<i32>)>{
    let mut left_data: HashSet<i32> = HashSet::new();
    let mut right_data: HashSet<i32> = HashSet::new();

    let file: File = File::open(file_path)?;
    let read_file: BufReader<File> = BufReader::new(file);

    for line in read_file.lines() {
        let line: String = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2{
            if let(Ok(left), Ok(right)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()){
                left_data.insert(left);
                right_data.insert(right);
            }else{
                eprintln!("Skipping malformed line: '{}'", line);
            }
        } else{
            eprintln!("Skipping malformed line: '{}'", line);
        }
    }
    let mut left_numbers: Vec<_> = left_data.into_iter().collect();
    let mut right_numbers: Vec<_> = right_data.into_iter().collect();
    left_numbers.sort();
    right_numbers.sort();
    Ok((left_numbers, right_numbers))
}

pub fn calculate_distance(left_numbers: Vec<i32>, right_numbers: Vec<i32>) -> i32{
    let mut distance: i32 = 0;
    let length_min = min(left_numbers.len(), right_numbers.len());

    for i in 0..length_min{
        distance += (left_numbers[i] - right_numbers[i]).abs();
    }
 
    distance
}