use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use std::collections::HashMap;
use day_01_lib::process_input::{split_data, calculate_distance, calculate_similarity_score, similarity_score_total};


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
    let (_dir_temp, file_path) = create_test_file("3 4\n4 3\n2 5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers, similarity_score) = split_data(&file_path).expect("Failed to split data");
    let mut expected_similarity_scores: HashMap<i32, i32> = HashMap::new();
    expected_similarity_scores.insert(1, 0);
    expected_similarity_scores.insert(2, 0);
    expected_similarity_scores.insert(3, 3);
    expected_similarity_scores.insert(4, 1);

    println!("Actual left_numbers: {:?}", left_numbers);
    println!("Actual right_numbers: {:?}", right_numbers);

    assert_eq!(left_numbers, vec![1,2,3,3,3,4]);
    assert_eq!(right_numbers, vec![3,3,3,4,5,9]);
    assert_eq!(similarity_score, expected_similarity_scores);

}

#[test]
fn test_split_data_invalid() {
    let (_dir_temp, file_path) = create_test_file("3 4\ninvalid 3\n2 5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers, _similarity_score) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, vec![1, 2, 3, 3, 3]);
    assert_eq!(right_numbers, vec![3, 3, 4, 5, 9]);
}

#[test]
fn test_split_data_negative() {
    let (_dir_temp, file_path)= create_test_file("3 4\n4 3\n2 -5\n1 3\n3 9\n3 3");
    let (left_numbers, right_numbers, _similarity_score) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, vec![1, 2, 3, 3, 3, 4]);
    assert_eq!(right_numbers, vec![-5, 3, 3, 3, 4, 9]);
}

#[test]
fn test_split_data_empty() {
    let (_dir_temp, file_path) = create_test_file("");
    let (left_numbers, right_numbers, _similarity_score) = split_data(&file_path).expect("Failed to split data");

    assert_eq!(left_numbers, Vec::new());
    assert_eq!(right_numbers, Vec::new());
}

#[test]
fn test_split_data_single_line() {
    let (_dir_temp, file_path) = create_test_file("7 8");
    let (left_numbers, right_numbers, _similarity_score) = split_data(&file_path).expect("Failed to split data");

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

#[test]
fn test_calculate_similarity_score() {
    let right_data = vec![3,3,1,3,4,5,9,1];
    let mut expected_similarity_scores: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score: HashMap<i32, i32> = HashMap::new();

    similarity_score.insert(1, 2);
    similarity_score.insert(2, 0);
    similarity_score.insert(3, 3);
    similarity_score.insert(4, 1);
    expected_similarity_scores.insert(1, 0);
    expected_similarity_scores.insert(2, 0);
    expected_similarity_scores.insert(3, 0);
    expected_similarity_scores.insert(4, 0);

    calculate_similarity_score(&mut expected_similarity_scores, &right_data);
    assert_eq!(similarity_score, expected_similarity_scores);

}

#[test]
fn test_similarity_score_total() {
    let mut similarity_scores: HashMap<i32, i32> = HashMap::new();
    similarity_scores.insert(1, 2);
    similarity_scores.insert(2, 0);
    similarity_scores.insert(3, 3);
    similarity_scores.insert(4, 1);

    let mut total: i32 = 0;
    similarity_score_total(&similarity_scores, &mut total);

    assert_eq!(total, 15);
}
