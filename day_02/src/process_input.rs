use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;
pub fn split_data(file_path: &str) -> Result<(Vec<i32>,Vec<i32>, HashMap<i32,i32>)> {
    let mut left_data: Vec<i32> = Vec::new();
    let mut right_data: Vec<i32> = Vec::new();
    let mut similarity_scores: HashMap<i32, i32> = HashMap::new();

    let file: File = File::open(file_path)?;
    let read_file: BufReader<File> = BufReader::new(file);

    for line in read_file.lines() {
        let line: String = line?;
        let pair: Vec<&str> = line.split_whitespace().collect();

        if pair.len() == 2{
            if let(Ok(left), Ok(right)) = (pair[0].parse::<i32>(), pair[1].parse::<i32>()){
                left_data.push(left);
                right_data.push(right);
                similarity_scores.entry(left).or_insert(0);
            }else{
                eprintln!("The pair data isn't integer: '{}'", line);
            }
        }else{
            eprintln!("The data isn't a pair: '{}'", line);
        }
    }
    left_data.sort();
    right_data.sort();
    calculate_similarity_score(&mut similarity_scores, &right_data);
    Ok((left_data, right_data, similarity_scores))
}

pub fn calculate_distance(left_numbers: Vec<i32>, right_numbers: Vec<i32>) -> i32{
    let mut distance: i32 = 0;
    for i in 0..left_numbers.len(){
        distance += (left_numbers[i] - right_numbers[i]).abs();
    }
    distance
}

pub fn calculate_similarity_score(similarity_score: &mut HashMap<i32, i32>, right_data: &[i32]){
    for &right in right_data{
        if let Some(value) = similarity_score.get_mut(&right){
            *value += 1;
        }
    }
}

pub fn similarity_score_total(similarity_scores: &HashMap<i32,i32>, total: &mut i32){
    for (key, value) in similarity_scores{
        *total += key*value;
    }
}
