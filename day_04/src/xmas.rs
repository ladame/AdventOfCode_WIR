use std::{fs::{self, read_to_string}, io};

pub struct Xmas {
    width: usize,
    height: usize,
    matrix: Vec<Vec<char>>,
    total_xmas: i32,
    positions: Vec<((usize,usize), (usize,usize))>,
}

impl Xmas{
    pub fn new(matrix: Vec<Vec<char>>, total:i32) -> Xmas {
        Xmas {
            width: matrix[0].len() as i32,
            height: matrix.len() as i32,
            data: matrix,
            total_xmas: total,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_total_xmas(&self) -> i32 {
        self.total_xmas
    }

    pub fn find_words(&mut self, words: &str){
        let words_to_chars : Vec<char> = words.chars().collect();
        let right: (isize,isize) = (0,1);
        let left: (isize,isize) = (0,-1);
        let up: (isize,isize) = (-1,0);
        let down: (isize,isize) = (1,0);
        let diagonal_down_right: (isize,isize) = (1,1);
        let diagonal_down_left: (isize,isize) = (1,-1);
        let diagonal_up_right: (isize,isize) = (-1,1);
        let diagonal_up_left: (isize,isize) = (-1,-1);

        let directions: [(isize,isize); 8] = [
            right,
            left,
            up,
            down,
            diagonal_down_right,
            diagonal_down_left,
            diagonal_up_right,
            diagonal_up_left,
        ];

        for row in 0..self.width{
            for col in 0..self.height{
                for &direction in &directions{
                    if let Some(pos) = self.check_word(&words_to_chars, (row,col), direction){
                        self.positions.push(pos);
                        self.total_xmas += 1;
                    }
                }
            }
        }

    }

    fn check_word(&mut self, words: &[char], start:(usize,usize), direction:(isize,isize)) -> Option<((usize,usize), (usize,usize))>{
        let mut current:(usize,usize) = start;
        for &character in words{
            let (row,col) = current;
            if row >= self.height || col >= self.width || self.matrix[row][col] != character {
                return None;
             }
             current = (
                (row as isize + direction.0) as usize ,
                (col as isize + direction.1) as usize,
             );

        }
        Some((start,current))
    }

} 
