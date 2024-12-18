use day_15_lib::read_warehouse::{init_warehouse,get_width_height_command};
use day_15_lib::warehouse::Warehouse;

#[test]
fn test_get_width_height_command() {
    let file_path = "tests/warehouse_simple_test.txt";
    
    let result = get_width_height_command(file_path);
    let (width, height, commands) = result.unwrap();
    assert_eq!(width, 10);
    assert_eq!(height, 10);
    assert_eq!(commands, vec![
        '<', 'v', 'v', '>', '^', 
        'v', 'v', 'v', '<', '<', 
        '>', '<', '>', 'v', 'v', 
        '<', '<', 'v', '<', '^', 
        '^', '>', '<', '^', '>', 
        '^', '>', '>', '<', '>', 
        '>', '^', '>', '>', '^', 
        '<', '>', '<', '^', '^', 
        '^', '^', '>', 'v', 'v', 
        'v', '^', '^', '>', '>'
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
        (7, vec![0, 9]), (0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 
        (4, vec![0, 9]), (6, vec![0, 9]), (8, vec![0, 9]), 
        (5, vec![0, 2, 9]), (1, vec![0, 9]), (9, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 
        (3, vec![0, 9]), (2, vec![0, 9])
    ].into_iter().collect::<std::collections::HashMap<_, _>>();
    let expected_robot_position: (i32,i32) = (4, 4);
    let expected_box_positions = vec![
        (1, 3), (1, 6), (1, 8), (2, 7), (3, 2), (3, 3), (3, 6), (3, 8), 
        (4, 3), (4, 7), (5, 1), (5, 5), (6, 1), (6, 4), (6, 7), (7, 2), 
        (7, 3), (7, 5), (7, 7), (7, 8), (8, 5)
    ];
    assert_eq!(warehouse.get_walls_positions().unwrap(), &expected_walls_positions);
    assert_eq!(warehouse.get_robot_positions(), Ok(expected_robot_position));
    assert_eq!(warehouse.box_positions, expected_box_positions);
}
