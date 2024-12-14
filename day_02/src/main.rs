use day_02_lib::process_input::{read_reports};

fn main() {
 
    println!("Welcome to Day 2!");
    let file_path: &str = "src/input/reports.txt";

    match read_reports(file_path){
        Ok(result) => println!("The number of valid reports is: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}
