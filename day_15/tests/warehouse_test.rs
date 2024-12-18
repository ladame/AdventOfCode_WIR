
use day_15_lib::warehouse::Warehouse;

#[test]
fn test_constructor() {
    let warehouse = Warehouse::new(10,5);
    assert_eq!(warehouse.get_width(), 10);
    assert_eq!(warehouse.get_height(), 5);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&0).unwrap().contains(&10), false);
}

#[test]
fn test_get_walls_positions() {
    let warehouse = Warehouse::new(10,5);
    for i in 0..warehouse.get_width() {
        assert_eq!(warehouse.get_walls_positions().unwrap().get(&0).unwrap().contains(&i), true);
        assert_eq!(warehouse.get_walls_positions().unwrap().get(&(5-1)).unwrap().contains(&i), true);
    }
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&(5-1)).unwrap().contains(&10), false);
    assert!(warehouse.get_walls_positions().unwrap().get(&5).is_none());
}

#[test]
fn test_set_init_position() {
    let mut warehouse = Warehouse::new(10,5);
    let content = "#O#..O.@.#";
    let row: i32 = 1;
    let target_correct: (i32,i32) = (1,1);
    let target_incorrect: (i32,i32) = (1,3);
    let target_incorrect_again: (i32,i32) = (1,4);
    warehouse.set_init_position(&row,content);
    println!{"Wall positions {:?}", warehouse.get_walls_positions()};
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&0), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&2), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&9), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&3), false);

    assert_eq!(warehouse.get_robot_positions().unwrap(), (row,7));
    println!{"Robot positions {:?}", warehouse.get_robot_positions()};

    println!{"Box positions {:?}", warehouse.box_positions};

    assert_eq!(warehouse.get_box_positions(&target_correct).unwrap(), true);
    assert_eq!(warehouse.get_box_positions(&target_incorrect).unwrap(), false);
    assert_eq!(warehouse.get_box_positions(&target_incorrect_again).unwrap(), false);

}