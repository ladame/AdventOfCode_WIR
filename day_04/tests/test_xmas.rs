use day_04_lib::xmas::Xmas;

#[test]
fn test_constructor() {
    let xmas = Xmas::new("tests/words_test.txt");
    assert_eq!(xmas.get_width(), 10);
    assert_eq!(xmas.get_height(), 10);
    assert_eq!(xmas.get_total_xmas(), 0);
}   

#[test]
fn test_find_words() {
    let mut xmas = Xmas::new("tests/words_test.txt");
    xmas.find_words("XMAS");
    assert_eq!(xmas.get_total_xmas(), 18);
}