
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
    println!{"Wall positions test {:?}", warehouse.get_walls_positions()};
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&0), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&2), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&9), true);
    assert_eq!(warehouse.get_walls_positions().unwrap().get(&row).unwrap().contains(&3), false);

    assert_eq!(warehouse.get_robot_positions().unwrap(), (7, row));
    //println!{"Robot positions {:?}", warehouse.get_robot_positions()};

    //println!{"Box positions {:?}", warehouse.box_positions};

    assert_eq!(warehouse.get_box_positions(&target_correct).unwrap(), true);
    assert_eq!(warehouse.get_box_positions(&target_incorrect).unwrap(), false);
    assert_eq!(warehouse.get_box_positions(&target_incorrect_again).unwrap(), false);

}

#[test]
fn test_move_right_down(){
    let mut warehouse = Warehouse::new(5,3);
    let mut content = "#...#";
    let mut row: i32 = 0;
    let mut robot_position: (i32,i32) = (2,1);
    let direction: Vec<char> = vec!['>','v'];
    warehouse.set_init_position(&row,content);
    content = "#.@.#";
    row = 1;
    warehouse.set_init_position(&row,content);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
    content = "#...#";
    row = 2;
    warehouse.set_init_position(&row,content);
    warehouse.move_robot(&direction);
    robot_position = (3,2);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
}


#[test]
fn test_move_right_wall_down(){
    let mut warehouse = Warehouse::new(5,3);
    let mut content = "#...#";
    let mut row: i32 = 0;
    let mut robot_position: (i32,i32) = (2,1);
    let direction: Vec<char> = vec!['>','v'];
    warehouse.set_init_position(&row,content);
    content = "#.@##";
    row = 1;
    warehouse.set_init_position(&row,content);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
    content = "#...#";
    row = 2;
    warehouse.set_init_position(&row,content);
    println!{"Wall positions {:?}", warehouse.get_walls_positions()};
    warehouse. move_robot(&direction);
    robot_position = (2,2);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
}


fn test_move_right_box_shift(){
    let mut warehouse = Warehouse::new(5,3);
    let mut content = "#...#";
    let mut box_position: (i32,i32) = (1,2);
    let mut robot_position: (i32,i32) = (1,1);
    let direction: Vec<char> = vec!['>','v'];
    let mut row = 1;
    content = "#@O.#";
    warehouse.set_init_position(&row,content);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
    content = "#...#";
    row = 2;
    warehouse.set_init_position(&row,content);
    warehouse. move_robot(&direction);
    robot_position = (1,2);
    assert_eq!(warehouse.get_robot_positions().unwrap(), robot_position);
 
}