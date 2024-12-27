use day_06_lib::{labyrinth::Labyrinth, guard::Guard};

#[test]
fn test_labyrinth_new(){
    let file_path: &str = "tests/map_test.txt";
    let map: Labyrinth = Labyrinth::new(file_path);
    assert_eq!(map.get_width(), 10);
    assert_eq!(map.get_height(), 10);
}

#[test]
fn test_labyrinth_find(){
    let file_path: &str = "tests/map_test.txt";
    let map: Labyrinth = Labyrinth::new(file_path);
    let guard: Guard = map.find_guard_position('^');
    assert_eq!(guard.get_position(), (6,4));
}

#[test]
fn test_labyrinth_get_position(){
    let file_path: &str = "tests/map_test.txt";
    let map: Labyrinth = Labyrinth::new(file_path);
    let guard: Guard = Guard::new(6,1);
    assert_eq!(map.get_wall(&guard), Some('#'));
}

#[test]
fn test_labyrinth_get_total_distinct_positions(){
    let file_path: &str = "tests/map_test.txt";
    let mut map: Labyrinth = Labyrinth::new(file_path);
    assert_eq!(map.get_total_distinct_positions(), 41);
}
