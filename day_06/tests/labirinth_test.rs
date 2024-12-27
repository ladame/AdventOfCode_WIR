use day_06_lib::labirinth::Labirinth;

#[test]
fn labirinth_new() {
    let file_path: &str = "tests/map_test.txt";
    let map: Labirinth = Labirinth::new(file_path).unwrap();
    assert_eq!(map.get_width(), 10);
    assert_eq!(map.get_height(), 10);
    assert_eq!(map.get_start_position(), (6,4));
    assert_eq!(map.get_obstacle_positions(), &[(1, vec![9]), (0, vec![4]), (7, vec![8]), (6, vec![1]), (3, vec![2]), (8, vec![0]), (9, vec![6]), (4, vec![7])].iter().cloned().collect());
}
#[test]
fn labirinth_up(){
    let file_path: &str = "tests/map_test.txt";
    let mut map: Labirinth = Labirinth::new(file_path).unwrap();
    let direction_up: (isize, isize) = (1, 0);
    assert_eq!(map.move_up(&direction_up), false);
    assert_eq!(map.get_start_position(), (1,4));
}

