use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use day_02_lib::process_input::{check_report, read_reports, check_consistency, report_correction};

   // 7 6 4 2 1  --> decreasing order SAFE
    // 1 2 7 8 9  --> increasing order but the max difference is 5 unsafe
    // 9 7 6 2 1  --> decreasing order but the max difference is 4 unsafe
    // 1 3 2 4 5  --> increasing order and decreasing order unsafe but if 2 is removed it will be SAFE
    // 8 6 4 4 1  --> decreasing order but the min difference is 0 and will be fixed if 4 is removed SAFE
    // 1 3 6 7 9  --> increasing order SAFE
    // 10 9 11 13 15

struct TestData{
    max_diff: i32,
}

fn setup() -> TestData{
    TestData{
        max_diff: 3,
    }

}

fn create_test_file(content: &str) -> (TempDir, String) {
    let dir_temp = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir_temp.path().join("test_reports.txt");
    let mut file_test = File::create(&file_path).expect("Failed to create test file");
    writeln!(file_test, "{}", content).expect("Failed to write to test file");
    let file_path_str = file_path.to_str().expect("Failed to convert file path to string").to_string();
    (dir_temp, file_path_str)
}

#[test]
fn test_read_reports(){
    let content = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    let (_dir_temp, file_path) = create_test_file(&content);
    let result = read_reports(&file_path).expect("Failed to read reports");
    assert_eq!(result, 4);
}

#[test]
fn test_check_report(){
    let mut report_decrease: Vec<i32> = vec![11, 9, 8, 5, 2];
    let mut report_increase: Vec<i32> = vec![2, 3, 4, 7, 9];
    let mut result : bool = check_report(&mut report_decrease);
    assert_eq!(result,true);
    result = check_report(&mut report_increase);
    assert_eq!(result,true);
}

#[test]
fn test_check_consistency(){
    let report_decrease: Vec<i32> = vec![7, 6, 4, 2, 1];
    let report_increase: Vec<i32> = vec![1, 3, 6, 7, 9];
    let report_increase_decrease: Vec<i32> = vec![1, 3, 6, 5, 2];
    let report_decrease_increase: Vec<i32> = vec![7, 6, 4, 5, 8];
    let data = setup();
    let mut result = check_consistency(&report_decrease, data.max_diff);
    assert_eq!(result, true);
    result = check_consistency(&report_increase, data.max_diff);
    assert_eq!(result, true);
    result = check_consistency(&report_increase_decrease, data.max_diff);
    assert_eq!(result, false);
    result = check_consistency(&report_decrease_increase, data.max_diff);
    assert_eq!(result, false);
}

#[test]
fn test_report_correction(){
    let report_duplicate: Vec<i32> = vec![8, 6, 4, 4, 1];
    let report_duplicate_twice: Vec<i32> = vec![6, 6, 4, 4, 1];
    let report_inc_dec: Vec<i32> = vec![1, 3, 2, 4, 5];
    let report_last_err: Vec<i32> = vec![44, 46, 49, 52, 55, 59];
    
    let data = setup();
    let mut result = report_correction(&report_duplicate, data.max_diff);
    assert_eq!(result, true);
    result = report_correction(&report_duplicate_twice, data.max_diff);
    assert_eq!(result, false);
    result = report_correction(&report_inc_dec, data.max_diff);
    assert_eq!(result, true);
    result = report_correction(&report_last_err, data.max_diff);
    assert_eq!(result, true);
  
}

#[test]
fn test_check_consistency_wrong_interval(){
    let report_decrease: Vec<i32> = vec![17, 6, 4, 2, 1];
    let report_increase: Vec<i32> = vec![1, 3, 6, 7, 19];
    let report_duplicate: Vec<i32> = vec![3, 3, 6, 5, 2]; 
    let report_inc_dec: Vec<i32> = vec![1, 3, 2, 4, 5];
    let data = setup();
    let mut result = check_consistency(&report_decrease, data.max_diff);
    assert_eq!(result, false);
    result = check_consistency(&report_increase, data.max_diff);
    assert_eq!(result, false);
    result = check_consistency(&report_duplicate, data.max_diff);
    assert_eq!(result, false);
    result = check_consistency(&report_inc_dec, data.max_diff);
    assert_eq!(result, false);
}