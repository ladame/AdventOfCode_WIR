use day_06_lib::labyrinth::Labyrinth;

fn main() {
    let file_path: &str = "src/input/map.txt";
    let mut map: Labyrinth = Labyrinth::new(file_path);
    println!("Total distinct positions: {}", map.get_total_distinct_positions());
}
