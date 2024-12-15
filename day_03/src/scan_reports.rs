use std::sync::LazyLock;


static DATA_OK: LazyLock<regex::Regex> = LazyLock::new(||
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());


pub fn scan_line(content: &str, total: &mut i32){

    for capture in DATA_OK.captures_iter(content){
        let number1: i32 = capture[1].parse().unwrap();
        let number2: i32 = capture[2].parse().unwrap();
        *total += number1 * number2;
    }

}
