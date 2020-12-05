use std::error::Error;
use std::{fs, io};

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("plane.txt")?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file()?;
    let mut highest_id = 0;
    for line in content.lines() {
        let rows = &line[0..7];
        let columns = &line[7..10];
        let rows = calculate_step(&rows.to_string(), 6, 127, 0);
        let columns = calculate_step(&columns.to_string(), 2, 7, 0);
        let id = rows * 8 + columns;
        if id > highest_id {
            highest_id = id;
        }
    }
    println!("Highest ID: {}", highest_id);
    Ok(())
}

fn calculate_step(code: &String, current_power: u32, upper_bound: i32, lower_bound: i32) -> i32 {
    if !code.is_empty() {
        let chars: Vec<char> = code.chars().collect();
        let updated_code = &code[1..code.len()].to_string();
        let updated_power = if current_power == 0 {
            0
        } else {
            current_power - 1
        };
        match chars[0] {
            'F' | 'L' => calculate_step(
                updated_code,
                updated_power,
                upper_bound - 2_i32.pow(current_power),
                lower_bound,
            ),
            'B' | 'R' => calculate_step(
                updated_code,
                updated_power,
                upper_bound,
                lower_bound + 2_i32.pow(current_power),
            ),
            _ => panic!(),
        }
    } else {
        if upper_bound != lower_bound {
            panic!("Bounds aren't equal!")
        } else {
            upper_bound
        }
    }
}
