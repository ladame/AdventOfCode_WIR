use std::collections::HashMap;

pub struct Warehouse {
    width: i32,
    height:i32,
    pub wall_positions: HashMap<i32,Vec<i32>>,
    robot_position: (i32,i32),
    pub box_positions: Vec<(i32, i32)>,
}

impl Warehouse {
        
    pub fn new(width: i32, height: i32) -> Warehouse {
        let mut warehouse = Warehouse {
            width,
            height,
            wall_positions: HashMap::new(),
            robot_position: (0,0),
            box_positions: Vec::new(),
        };
        warehouse.set_top_bottom_walls();
        warehouse
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn set_top_bottom_walls(&mut self){
        let mut wall_positions = Vec::new();
        for i in 0..self.width {
            wall_positions.push(i);
        }
        self.wall_positions.insert(0, wall_positions.clone());
        self.wall_positions.insert(self.height-1, wall_positions);
    }

    pub fn get_walls_positions(&self) -> Result<&HashMap<i32, Vec<i32>>, &'static str> {
        if self.wall_positions.is_empty() {
            Err("No wall positions found")
        } else {
            Ok(&self.wall_positions)
        }
    }

    pub fn get_robot_positions(&self) -> Result<(i32, i32), &'static str> {
        if self.robot_position.0 < 0 || self.robot_position.0 >= self.height || self.robot_position.1 < 0 || self.robot_position.1 >= self.width {
            Err("Robot position is out of bounds")
        } else {
            Ok(self.robot_position)
        }
    }

    pub fn get_box_positions(&self, target: &(i32, i32)) -> Result<bool, &'static str> {
        if target.0 < 0 || target.0 >= self.height || target.1 < 0 || target.1 >= self.width {
            Err("Target position is out of bounds")
        } else {
            Ok(self.box_positions.contains(target))
        }
    }

    pub fn set_init_position(&mut self, row: &i32, content: &str){
        if *row < 0{
            eprintln!("Row position is out of bounds {}", row);
        }
        
        let mut wall_list: Vec<i32> = Vec::new();
        for (x, ch) in content.chars().enumerate() {
            let x = x as i32;
            if x < 0 || x >= self.width {
                eprintln!("Column position is out of bounds");
            }
            match ch {
                '#' => {
                    wall_list.push(x);
                    self.wall_positions.insert(*row, wall_list.clone());
                },
                '@' => {
                    self.robot_position = (*row, x);
                },
                'O' => {
                    self.box_positions.push((*row, x));
                },
                _ => {}
            }
        }
        
    }
    
}