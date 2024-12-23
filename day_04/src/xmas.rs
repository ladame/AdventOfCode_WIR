

pub struct Xmas {
    width: i32,
    height:i32,
    data: Vec<Vec<char>>,
    total_xmas: i32,
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

    pub fn get_width(&self) -> i32 {
        self.width
    }   

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_total_xmas(&self) -> i32 {
        self.total_xmas
    }
}
