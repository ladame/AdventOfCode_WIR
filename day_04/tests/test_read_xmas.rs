use day_04_lib::read_xmas::read_xmas;


#[test]
fn test_read_xmas() {
    let file_path: &str = "tests/words_test.txt";
    
    assert_eq!(read_xmas(file_path).unwrap() , 18);
}