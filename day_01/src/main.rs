
use day_01_lib::process_input::{split_data, calculate_distance, similarity_score_total};
use std::{fs::read_to_string, time::Instant};

fn simple_way(file_path: &str){
    let start = Instant::now();
    let data = read_to_string(file_path).expect("Unable to read file");
    let (mut left, mut right): (Vec<_>, Vec<_>) = data.lines().map(|line| {
        let mut part = line.split_whitespace().map(|p| p.parse::<usize>().unwrap()); 
        (part.next().unwrap(), part.next().unwrap())})
        .unzip();
    left.sort();
    right.sort();

    let distance: usize = left.iter().zip(right.iter()).map(|(l,r)| l.abs_diff(*r)).sum();
    println!("Distance: {:?}", distance);
    let duration = start.elapsed();
    println!("Finished in {:?}", duration);

}
fn main() {
    // group1 = [3,4,2,1,3,3];--> 1,2,3,3,3,4
    // group2 = [4,3,5,3,9,3];--> 3,3,3,4,5,9
    // distance = 2+1+1+2+5=11;

    println!("Welcome to Day 1!");
    let start = Instant::now();
    // Make list of group 1 and group 2
    let file_path: &str = "src/input/file_location.txt";

    match  split_data(file_path){
        Ok((left_number, right_number, similarity_scores)) => {
            let distance: i32 = calculate_distance(left_number, right_number);
            let mut total: i32 = 0;
            println!("Distance: {:?}", distance);
            //similarity_score_total(&similarity_scores, &mut total);
            //println!("Similarity Scores: {:?}", total);
        }
        Err(e) => eprintln!("Error reading file '{}': {}", file_path, e),
    }
    let duration = start.elapsed();
    println!("Finished in {:?}", duration);
    simple_way(file_path);


}