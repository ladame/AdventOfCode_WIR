use day_03_lib::read_reports::{read_reports, read_reports_do};


fn main() {
    println!("Welcome to Day 3!");
    let file_path: &str = "src/input/numbers.txt";

    match read_reports(file_path) {
        Ok(result) => println!("The result of multiplying the numbers is {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match read_reports_do(file_path) {
        Ok(result) => println!("The result of multiplying the numbers is {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
