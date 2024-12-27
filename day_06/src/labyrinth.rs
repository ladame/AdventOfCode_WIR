use std::{fs::read_to_string, io::Result};
use std::collections::HashSet;
use crate::guard::Guard;

pub struct Labyrinth{
    width: isize,
    height: isize,
    visited: HashSet<Guard>,
    matrix: Vec<Vec<char>>,
}

impl Labyrinth{
    pub fn new(file_path: &str) -> Labyrinth{
        let data: String = read_to_string(file_path).expect("Check the file path!!!");
        let matrix: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        Self{
            width: matrix.first().unwrap().len() as isize,
            height: matrix.len() as isize,
            visited: HashSet::new(),
            matrix,
        }
    }

    pub fn get_width(&self) -> isize{
        self.width
    }   

    pub fn get_height(&self) -> isize{
        self.height
    }  

    pub fn get_total_distinct_positions(&mut self) -> usize{
        let mut guard: Guard = self.find_guard_position('^');
        self.run_guard(&mut guard);
        self.visited.len()
    }

    pub fn find_guard_position(&self, character: char) -> Guard{
        for r in 0..self.height{
            for c in 0..self.width{
                if self.matrix[r as usize][c as usize] == character{
                    return Guard::new(r,c);
                }
            }
        }
        panic!("Guard has run away!!!");
    }

    pub fn get_wall(&self, guard: &Guard) -> Option<char>{
        if guard.get_position().0 < 0 || guard.get_position().0 >= self.height 
            || guard.get_position().1 < 0 || guard.get_position().1 >= self.width{
            return None;
        }
        Some(self.matrix[guard.get_position().0 as usize][guard.get_position().1 as usize])
    }

    fn run_guard(&mut self, guard: &mut Guard){
       
        let mut direction: Guard = Guard::new(-1,0);
        loop{
            
            self.visited.insert(*guard);
            let next = guard.add(&direction);
            if let Some(c) = self.get_wall(&next){
                if c == '#'{
                    direction = direction.right();
                    //println!("Guard position: {:?}", guard.get_position());
                }else{
                    *guard = next;
                }
            }else {
                break;
            }
        }
    }
}