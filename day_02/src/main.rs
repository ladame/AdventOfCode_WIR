use std::fs::read_to_string;
use std::time::Instant;

use day_02_lib::process_input::{read_reports};

fn longer_way(file_path: &str){
    let start = Instant::now();
    let data: String = read_to_string(file_path).expect("Unable to read file, check the path!");
    let valid: usize = data.lines().filter(|line| {
     
    let numbers: Vec<_> = line.split_whitespace().map(|n| n.parse::<isize>().unwrap()).collect();
    let mut numbers_vec: Vec<Vec<_>> = (0..numbers.len())
    .map(
        |i| {
            let mut new_numbers=numbers.clone();
            new_numbers.remove(i);
            new_numbers
        }
    ).collect();
    numbers_vec.push(numbers);
    numbers_vec.iter().any(
        |numbers| {
            numbers.windows(3).all(
                |w| {
                    let d0 = w[2] - w[1];
                    let d1 = w[1] - w[0];
                    d0.is_positive() == d1.is_positive()
                    // to filter same value such as 1,1,1
                    && d0.abs() > 0
                    && d0.abs() < 4
                    && d1.abs() > 0
                    && d1.abs() < 4
                })
        })
     /*    numbers.windows(3).all(
            |w| {
                let d0 = w[1] - w[0];
                let d1 = w[2] - w[1];
                d0.is_positive() == d1.is_positive()
        
                && d0.abs() < 4
    
                && d1.abs() < 4
            })*/
    }).count();
    println!("The number of valid reports is: {}", valid);
    let finish = start.elapsed();
    println!("Duration: {:.2?}", finish);
}
fn main() {
    
    println!("Welcome to Day 2!");
    let start = Instant::now();
    let file_path: &str = "src/input/reports.txt";

    match read_reports(file_path){
        Ok(result) => println!("The number of valid reports is: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    let finish = start.elapsed();
    println!("Duration: {:.2?}", finish);
    longer_way(file_path);
}
