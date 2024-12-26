use day_05_lib::manual::Manual;

fn main() {
    let file_path: &str = "src/input/sequences.txt";
    //let file_path: &str = "tests/test_sequences.txt";
    let manual: Manual = Manual::new(file_path).unwrap();
    println!("Total pages: {}", manual.get_sum_pages());
}
