use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::warehouse::Warehouse;

pub fn read_warehouse(file_path: &str) -> Result<i32, io::Error> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let  height: i32;
    let  width: i32;
    let mut commands: Vec<char> = Vec::new();
    let mut sum_of_boxes: i32 =0;
    
    match get_width_height_command(file_path) {
        Ok((w, h, c)) => {
            width = w;
            height = h;
            commands = c;

        },
        Err(e) => return Err(e),
    };
    let mut warehouse: Warehouse = Warehouse::new(width, height);
    // initialize data in the warehouse
    init_warehouse(file_path, &mut warehouse);
    // Move the box

    Ok(sum_of_boxes)
}

// open the file to get the height, width and the commands
pub fn get_width_height_command(file_path: &str) -> Result<(i32, i32, Vec<char>), io::Error> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    
    first_line = first_line.trim_start().to_string();
    first_line = first_line.trim_end().to_string();

    let width = first_line.len() as i32;
    let mut height: i32 = 1;
    let directions = ">^<v";
    let mut commands: Vec<char> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("#"){
            height += 1;
            continue;
        }
        for ch in line.chars() {
            if directions.contains(ch) {
                commands.push(ch);
            }
        }
    }

    Ok((width, height, commands))

}

pub fn init_warehouse(file_path:&str, warehouse: &mut Warehouse){
    // open the file
    let file: File = match File::open(file_path){
        Ok(file) => file,
        Err(e) => {
            println!("Error opening the file file to initialization: {}", e);
            return;
        },
    };
    let reader: BufReader<File> = BufReader::new(file);
    let mut row: i32 = 0;
    for line in reader.lines() {
        warehouse.set_init_position(&row, &line.unwrap());
        row +=1;
    }
}
