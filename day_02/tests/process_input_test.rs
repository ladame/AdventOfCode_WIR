use std::fs::File;
use std::io::Write;
use tempfile::TempDir;
use day_02_lib::process_input::{check_report, check_decrease, check_increase, read_reports};

   // 7 6 4 2 1  --> decreasing order safe
    // 1 2 7 8 9  --> increasing order but the max difference is 5 unsafe
    // 9 7 6 2 1  --> decreasing order but the max difference is 4 unsafe
    // 1 3 2 4 5  --> increasing order and decreasing order unsafe
    // 8 6 4 4 1  --> decreasing order but the min difference is 0 unsafe
    // 1 3 6 7 9  --> increasing order safe

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
    assert_eq!(result, 2);
}

#[test]
fn test_read_reports_less_than_five_column(){
    let content = "7 6 4 2\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    let (_dir_temp, file_path) = create_test_file(&content);
    let result = read_reports(&file_path).expect("Failed to read reports");
    assert_eq!(result, 1);
}

#[test]
fn test_read_reports_more_than_five_column(){
    let content = "7 6 4 3 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    let (dir_temp, file_path) = create_test_file(&content);
    let result = read_reports(&file_path).expect("Failed to read reports");
    assert_eq!(result, 1);
}

#[test]
fn test_check_report_no_diff(){
    let report: [i32; 5] = [7, 7, 4, 2, 1];
    let result : bool = check_report(&report);
    assert_eq!(result,false);
}

#[test]
fn test_check_decrease_correct(){
    let report: [i32; 5] = [7, 6, 4, 2, 1];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,true);
}

#[test]
fn test_check_decrease_incorrect(){
    let report: [i32; 5] = [7, 9, 4, 2, 1];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_decrease_more_than_max_diff(){
    let report: [i32; 5] = [15, 9, 4, 2, 1];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_decrease_no_diff(){
    let report: [i32; 5] = [10, 10, 7, 6, 5];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_decrease_no_diff_in_the_end(){
    let report: [i32; 5] = [10, 9, 8, 4, 4];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_decrease_then_increase(){
    let report: [i32; 5] = [10, 10, 9, 10, 1];
    let max_diff: i32 = 3;
    let result : bool = check_decrease(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_increase_correct(){
    let report: [i32; 5] = [1, 2, 4, 6, 7];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,true);
}

#[test]
fn test_check_increase_incorrect(){
    let report: [i32; 5] = [5, 2, 4, 6, 7];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_increase_more_than_max_diff(){
    let report: [i32; 5] = [1, 2, 4, 6, 17];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_increase_no_diff(){
    let report: [i32; 5] = [1, 2, 2, 6, 7];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_increase_no_diff_in_the_end(){
    let report: [i32; 5] = [1, 2, 4, 7, 7];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,false);
}

#[test]
fn test_check_increase_then_decrease(){
    let report: [i32; 5] = [1, 2, 4, 3, 7];
    let max_diff: i32 = 3;
    let result : bool = check_increase(&report, max_diff);
    assert_eq!(result,false);
}