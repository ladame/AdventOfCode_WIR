use std::collections::HashMap;
use std::sync::LazyLock;
use regex::Regex;

static WALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r".\*#.\.").unwrap()
});

static ROBOT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r".\*@.\.").unwrap()
});

static BOX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r".\*0.\.").unwrap()
});
pub struct Warehouse {
    width: i32,
    height:i32,
    wall_positions: HashMap<i32,Vec<i32>>,
    robot_position: (i32,i32),
    box_positions: Vec<(i32, i32)>,
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

    pub fn get_walls_positions(&self) -> &HashMap<i32,Vec<i32>> {
        &self.wall_positions
    }

    pub fn set_walls_position(&mut self, row: &i32, content: &str){
        for capture in WALL.captures_iter(content){
            let x: i32 = capture[1].parse().unwrap();
//            let y: i32 = capture[2].parse().unwrap();
            let a = &self.wall_positions.get_mut(&row).unwrap().push(x);
            println!("Wall positions {:?}", self.wall_positions);
            eprintln!("aaaaaaaaa {:?}", a);
        }
    }
    
}