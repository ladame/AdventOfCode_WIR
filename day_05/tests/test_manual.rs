use day_05_lib::manual::Manual;

#[test]
fn test_manual() {
    let file_path: &str = "tests/test_sequences.txt";
    let manual = Manual::new(file_path);
    assert_eq!(manual.is_ok(), true);
}

#[test]
fn test_set_rules() {
    let file_path: &str = "tests/test_sequences.txt";
    let manual = Manual::new(file_path).unwrap();
    assert_eq!(manual.get_rules().len(), 21);
}

#[test]
fn test_set_updates() {
    let file_path: &str = "tests/test_sequences.txt";
    let manual = Manual::new(file_path).unwrap();
    assert_eq!(manual.get_updates().len(), 6);
}

#[test]
fn test_sum_pages() {
    let file_path: &str = "tests/test_sequences.txt";
    let manual = Manual::new(file_path).unwrap();
    assert_eq!(manual.get_sum_pages(), 143);
}
