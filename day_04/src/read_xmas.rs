use std::{fs, io};
use crate::xmas::{self, Xmas};


pub fn read_xmas(file_path: &str) -> Result<i32, io::Error>{
    let mut matrix: Vec<Vec<char>> = Vec::new();

    let _ = fill_matrix(&file_path, &mut matrix);

//    for line in contents.lines(){
        // Check the matrix size of the input file
        // Find word Xmas horizontally
        // Find word Xmas horizontally backwards
        // Find words Xmas vertically
        // Find words Xmas vertically backwards
        // Find words Xmas diagonally
        // Find words Xmas diagonally backwards
//    }
    Ok(1000)
}

fn fill_matrix(file_path: &str, matrix: &mut Vec<Vec<char>>) -> Result<i32, io::Error>{
    let data = fs::read_to_string(file_path).unwrap();
    *matrix = data.lines().map(|line| line.chars().collect()).collect();
    let mut check_index_horizontal: i8 = 0;
    let mut check_index_horizontal_reverse: i8 = 0;
    let mut total_xmas: i32 = 0;

    for i in 0..matrix.len(){
        for j in 0..matrix[0].len(){
            print!("{}", matrix[i][j]);
            check_horizontal(&mut check_index_horizontal, matrix[i][j]);
            check_horizontal_reverse(&mut check_index_horizontal_reverse, matrix[i][j]);
            if check_index_horizontal == 4 {
                total_xmas += 1;
                check_index_horizontal = 0;
            }
            if check_index_horizontal_reverse == 4 {
                total_xmas += 1;
                check_index_horizontal_reverse = 0;
            }
        }
    };
    
    Ok(total_xmas)
}

fn check_horizontal(check_index: &mut i8, word: char){
    if (*check_index == 0 && word == 'X') || (*check_index == 1 && word == 'M') || (*check_index == 2 && word == 'A') || (*check_index == 3 && word == 'S'){
        *check_index += 1;
    }
    else if *check_index > 0 && word == 'X'{
        print!("RESET {} ", word);
        *check_index = 1;
    }else{
        *check_index = 0;
    }
}

fn check_horizontal_reverse(check_index: &mut i8, word: char){
    if (*check_index == 0 && word == 'S') || (*check_index == 1 && word == 'A') || (*check_index == 2 && word == 'M') || (*check_index == 3 && word == 'X'){
        *check_index += 1;
    }
    else if *check_index > 0 && word == 'X'{
        print!("RESET {} ", word);
        *check_index = 1;
    }else{
        *check_index = 0;
    }
}
#[cfg(test)]
mod test_read_xmas{
    use super::*;

    #[test]
    fn test_check_horizontal(){
        let line: String = "MMMSXXMASMMMMSXXMASMMMMSXXMASM".to_string();
        let mut check_index: i8 = 0;
        let mut total_xmas: i8 = 0;
        for word in line.chars(){
            check_horizontal(&mut check_index, word);
            println!("check_index: {}", check_index);
            if check_index == 4{
                total_xmas += 1;
                check_index = 0;
            }
        }        
        assert_eq!(total_xmas, 3);
    }

    #[test]
    fn test_check_horizontal_reverse(){
        let line: String = "XMASAMXAMMXMASAMXAMM".to_string();
        let mut check_index: i8 = 0;
        let mut total_xmas: i8 = 0;
        for word in line.chars(){
            check_horizontal_reverse(&mut check_index, word);
            println!("check_index: {}", check_index);
            if check_index == 4{
                total_xmas += 1;
                check_index = 0;
            }
        }        
        assert_eq!(total_xmas, 2);
    }

    #[test]
    fn test_fill_matrix(){
        let file_path: &str = "tests/words_test.txt";
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let total_xmas: i32 = fill_matrix(&file_path, &mut matrix).unwrap();
        let xmas: Xmas = Xmas::new(matrix,total_xmas);
        assert_eq!(xmas.get_width(), 10);
        assert_eq!(xmas.get_height(), 10);
        assert_eq!(xmas.get_total_xmas(), 5);
    }
}
