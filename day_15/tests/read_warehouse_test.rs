use day_15_lib::read_warehouse::{init_warehouse,get_width_height_command};
use day_15_lib::warehouse::Warehouse;

#[test]
fn test_get_width_height_command() {
    let file_path = "tests/warehouse_simple_test.txt";
    
    let result = get_width_height_command(file_path);
    let (width, height, commands) = result.unwrap();
    assert_eq!(width, 8);
    assert_eq!(height, 8);
    assert_eq!(commands, vec![
        '<', '^', '^', '>', '>', '>', 'v', 'v', '<', 'v', '>', '>', 'v', '<', '<'
    ]);
}

#[test]
fn test_init_warehouse(){
    let file_path = "tests/warehouse_simple_test.txt";
    let result = get_width_height_command(file_path);
    let (width, height, commands) = result.unwrap();
    let mut warehouse = Warehouse::new(width,height);
    init_warehouse(&file_path,&mut warehouse);
    assert_eq!(warehouse.get_width(), width);
    assert_eq!(warehouse.get_height(), height);
    let expected_walls_positions = vec![
        (4, vec![0, 2, 7]), (3, vec![0, 7]), (1, vec![0, 7]), 
        (5, vec![0, 7]), (7, vec![0, 1, 2, 3, 4, 5, 6, 7]), 
        (6, vec![0, 7]), (2, vec![0, 1, 7]), (0, vec![0, 1, 2, 3, 4, 5, 6, 7])
    ].into_iter().collect::<std::collections::HashMap<_, _>>();
    let expected_robot_position: (i32,i32) = (2, 2);
    let expected_box_positions = vec![
        (3,1), (5,1), (4,2), (4,3), (4, 4), (4,5)
    ];
    assert_eq!(warehouse.get_walls_positions().unwrap(), &expected_walls_positions);
    assert_eq!(warehouse.get_robot_positions(), Ok(expected_robot_position));
    assert_eq!(warehouse.box_positions, expected_box_positions);
}
