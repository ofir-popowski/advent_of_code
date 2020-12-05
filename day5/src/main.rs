use std::error::Error;
use std::{fs, io};

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("plane.txt")?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file()?;
    let mut highest_id = 0;
    let mut existing_ids: Vec<i32> = Vec::new();
    for line in content.lines() {
        let binary_string: String = line
            .chars()
            .map(|c| match c {
                'F' | 'L' => '0',
                'B' | 'R' => '1',
                _ => panic!(),
            })
            .collect();
        let id = i32::from_str_radix(&binary_string, 2).unwrap();
        existing_ids.push(id);
        if id > highest_id {
            highest_id = id;
        }
    }
    existing_ids.sort();
    existing_ids.iter().enumerate().for_each(|(i, _x)| {
        if i < existing_ids.len() - 1 {
            let lower = existing_ids[i];
            let higher = existing_ids[i + 1];
            if higher - lower == 2 {
                println!("Available seat: {}", higher - 1);
            }
        }
    });
    println!("Highest ID: {}", highest_id);
    Ok(())
}
