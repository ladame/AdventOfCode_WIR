use day_02_lib::process_input::{split_data, calculate_distance, similarity_score_total};

fn main() {
    // group1 = [3,4,2,1,3,3];--> 1,2,3,3,3,4
    // group2 = [4,3,5,3,9,3];--> 3,3,3,4,5,9
    // similarity score = (3*3) + (4*1) + (2*0) + (1*0) + (3*3) + (3*3)
    // similarity score = 9+4+0+0+9+9 = 31
    println!("Welcome to Day 2!");
    let file_path: &str = "src/input/file_location.txt";

    match split_data(file_path){
        Ok((left_number, right_number, similarity_score)) => {
            let distance: i32 = calculate_distance(left_number, right_number);
            let mut total:i32 = 0;
            println!("Distance: {:?}", distance);
            similarity_score_total(&similarity_score, &mut total);
            println!("similarity_score: {:?}", total);
        }
        Err(e) => eprintln!("Error reading file '{}'; {}", file_path, e),
    }

}
