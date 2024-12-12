
use day_01_lib::process_input::{split_data, calculate_distance};
fn main() {
    // location = [chief_office, id1, id2,....]
    // group1 = [3,4,2,1,3,3];
    // group2 = [4,3,5,3,9,3];
    // pair = [(1,3), (2,4), (3,5), (4,9)];
    // distance = 2+2+2+5=11;
    println!("Welcome to Day 1!");

    // Make list of group 1 and group 2
    let file_path: &str = "src/input/file_location.txt";

    match  split_data(file_path){
        Ok((left_number, right_number)) => {
            let distance: i32 = calculate_distance(left_number, right_number);
            println!("Distance: {:?}", distance);
        }
        Err(e) => eprintln!("Error reading file '{}': {}", file_path, e),
    }

}

