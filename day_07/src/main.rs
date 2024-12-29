use std::{fs::{read, read_to_string}, vec};


fn main() {
    let file_path: &str = "src/input/data.txt";
    let data: String = read_to_string(file_path).expect("Check the file path!");
    let sum: usize = data.lines().map(|line| {
        let mut split = line.split(':');
        let target: usize = split.next().unwrap().parse::<usize>().unwrap();
        let numbers: Vec<usize> = split.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        (target, numbers)   
    }).filter(|x| can_solve(x.0, &x.1)).map(|x| x.0).sum();

    println!("The sum is: {}", sum);
}

fn can_solve(target: usize, numbers: &[usize]) -> bool {
    let mut result = vec![numbers[0]];
    for number in &numbers[1..]{
        result = result
        .iter()
        .flat_map(|result| vec![result * number, result + number])
        .collect();
    }
    result.iter().any(|&result| result == target)
}

