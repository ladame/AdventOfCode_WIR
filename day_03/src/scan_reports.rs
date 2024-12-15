use std::sync::LazyLock;


static DATA_DO: LazyLock<regex::Regex> = LazyLock::new(||
    regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

static DATA_DONT: LazyLock<regex::Regex> = LazyLock::new(||
    regex::Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap());

pub fn scan_line(content: &str, total: &mut i32){

    for capture in DATA_DO.captures_iter(content){
        let number1: i32 = capture[1].parse().unwrap();
        let number2: i32 = capture[2].parse().unwrap();
        *total += number1 * number2;
    }
}

pub fn scan_line_do(content: &str, total: &mut i32){
    let mut enabled: bool = true;
    for capture in DATA_DONT.captures_iter(content) {
        match &capture[1] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let number1: i32 = capture[2].parse().unwrap();
                    let number2: i32 = capture[3].parse().unwrap();
                    *total += number1 * number2;
                }
            }
        }
    }



}