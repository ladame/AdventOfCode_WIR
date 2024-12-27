use std::{collections::HashMap, fs::read_to_string, io};

pub struct Labirinth {
    width: usize,
    height: usize,
    total_distinct_positions: i32,
    start_position: (usize, usize),
    obstacle_positions: HashMap<usize, Vec<usize>>,
    matrix: Vec<Vec<char>>,
}

impl Labirinth {
    pub fn new(file_path: &str) -> Result<Self, io::Error> {
        let content: String = read_to_string(file_path).expect("Cannot open file, check the path!");
        let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        let labirinth: Labirinth = Labirinth::scan_matrix(&mut grid);

        Ok(Self {
            width: labirinth.get_width(),
            height: labirinth.get_height(),
            total_distinct_positions: 0,
            start_position: labirinth.get_start_position(),
            obstacle_positions: labirinth.obstacle_positions,
            matrix: grid,
        })

    }
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_start_position(&self) -> (usize, usize) {
        self.start_position
    }

    pub fn get_obstacle_positions(&self) -> &HashMap<usize, Vec<usize>> {
        &self.obstacle_positions
    }

    fn scan_matrix(grid: &mut Vec<Vec<char>>) -> Labirinth {
        let width: usize = grid[0].len();
        let height: usize = grid.len();
        let mut start_position: (usize, usize) = (0, 0);
        let mut obstacle_positions: HashMap<usize, Vec<usize>> = HashMap::new();

        for row in 0..height {
            for col in 0..width {
                if grid[row][col] == '^' {
                    start_position = (row, col);
                } else if grid[row][col] == '#' {
                    if obstacle_positions.contains_key(&row) {
                        obstacle_positions.get_mut(&row).unwrap().push(col);
                    } else {
                        obstacle_positions.insert(row, vec![col]);
                    }
                }
            }
        }

        let labyrinth: Labirinth = Labirinth {
            width: width,
            height: height,
            total_distinct_positions: 0,
            start_position: start_position,
            obstacle_positions: obstacle_positions,
            matrix: grid.clone(),
        };
        return labyrinth
}


pub fn move_up(&mut self, direction: &(isize,isize)) -> bool{
    let value_to_find: usize = self.start_position.1;
    for row in (0..self.start_position.0).rev(){
        match self.obstacle_positions.get(&row){
            Some(value) => {
                if value.contains(&value_to_find){
                    println!("row: {}",row);
                    self.start_position.0 = (row as isize + 1) as usize;
                    return false;
                }
            },
            None => continue,
        }
    };
    return true;
}

pub fn moving_around(&mut self, start: (usize, usize), finish: bool, direction: (isize, isize))
{
   if finish == true{

    if start == self.start_position || direction == (-1,0){
        self.move_up(&direction);
    }
}

}
}