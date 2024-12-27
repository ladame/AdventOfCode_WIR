#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guard{
    row: isize,
    column: isize,
}

impl Guard{
    pub fn new(row:isize, column:isize) -> Guard{
        Self{row, column}
    }

    pub fn get_position(&self) -> (isize, isize){
        (self.row, self.column)
    }

    pub fn right(&self) -> Self{
        Guard::new(self.column, -self.row)

    }

    pub fn add(&self, other: &Self) -> Self{
        Self { 
            row: self.row + other.row, 
            column: self.column + other.column 
        }
    }

}