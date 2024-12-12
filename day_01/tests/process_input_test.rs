use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use day_01_lib::process_input::{split_data, calculate_distance};


fn create_test_file(content: &str) -> (TempDir, String) {
    let dir_temp = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir_temp.path().join("test_numbers.txt");
    let mut file_test = File::create(&file_path).expect("Failed to create test file");
    writeln!(file_test, "{}", content).expect("Failed to write to test file");
    let file_path_str = file_path.to_str().expect("Failed to convert file path to string").to_string();
    (dir_temp, file_path_str)
}


#[test]
fn test_split_data() {
    let (dir_temp, file_path) = create_test_file("3 4\n4 3\n2 5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers) = split_data(&file_path).expect("Failed to split data");

    println!("Actual left_numbers: {:?}", left_numbers);
    println!("Actual right_numbers: {:?}", right_numbers);

    assert_eq!(left_numbers, vec![1,2,3,4]);
    assert_eq!(right_numbers, vec![3,4,5,9]);
}

#[test]
fn test_split_data_invalid() {
    let (dir_temp, file_path) = create_test_file("3 4\ninvalid 3\n2 5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, vec![1, 2, 3]);
    assert_eq!(right_numbers, vec![3, 4, 5, 9]);
}

#[test]
fn test_split_data_negative() {
    let (dir_temp, file_path)= create_test_file("3 4\n4 3\n2 -5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, vec![1, 2, 3, 4]);
    assert_eq!(right_numbers, vec![-5, 3, 4, 9]);
}

#[test]
fn test_split_data_empty() {
    let (dir_temp, file_path) = create_test_file("");
    let (left_numbers, right_numbers) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, Vec::new());
    assert_eq!(right_numbers, Vec::new());
}

#[test]
fn test_split_data_single_line() {
    let (dir_temp, file_path) = create_test_file("7 8");
    let (left_numbers, right_numbers) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, vec![7]);
    assert_eq!(right_numbers, vec![8]);
}

#[test]
fn test_calculate_distance() {
    let left_numbers = vec![1,2,3,4];
    let right_numbers = vec![3,4,5,9];

    let distance: i32 = calculate_distance(left_numbers, right_numbers);

    assert_eq!(distance, 11);
}

#[test]
fn test_calculate_distance_empty() {
    let left_numbers = Vec::new();
    let right_numbers = Vec::new();

    let distance: i32 = calculate_distance(left_numbers, right_numbers);

    assert_eq!(distance, 0);
}