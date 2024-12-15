use day_04_lib::read_xmas::read_xmas;
fn main() {
    let file_path: &str = "src/input/words.txt";
    println!("Welcome to Day 4!");
    match read_xmas(file_path){
        Ok(result) => println!("Total XMAS words are: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
