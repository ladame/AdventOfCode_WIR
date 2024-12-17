
use day_15_lib::warehouse::Warehouse;

#[test]
fn test_constructor() {
    let mut warehouse = Warehouse::new(10,5);
    assert_eq!(warehouse.get_width(), 10);
    assert_eq!(warehouse.get_height(), 5);
    assert_eq!(warehouse.get_walls_positions().get(&0).unwrap().contains(&10), false);
}

#[test]
fn test_get_walls_positions() {
    let mut warehouse = Warehouse::new(10,5);
    println!{"Wall positions {:?}", warehouse.get_walls_positions()};
    for i in 0..warehouse.get_width() {
        assert_eq!(warehouse.get_walls_positions().get(&0).unwrap().contains(&i), true);
        assert_eq!(warehouse.get_walls_positions().get(&(5-1)).unwrap().contains(&i), true);
    }
}

#[test]
fn test_set_walls_position() {
    let mut warehouse = Warehouse::new(10,5);
    let row: i32 = 1;
    warehouse.set_walls_position(&row,"#O#..O...#");
    assert_eq!(warehouse.get_walls_positions().get(&row).unwrap().contains(&0), true);
    let walls_in_column_0 = warehouse.get_walls_positions().get(&row).unwrap();
    println!("Walls in column {}: {:?}", row, walls_in_column_0);
    println!("Wall positions {} {:?}", row, warehouse.get_walls_positions().get(&row).unwrap().contains(&0));
    assert_eq!(warehouse.get_walls_positions().get(&row).unwrap().contains(&2), true);
    assert_eq!(warehouse.get_walls_positions().get(&row).unwrap().contains(&9), true);
    assert_eq!(warehouse.get_walls_positions().get(&row).unwrap().contains(&3), false);

}