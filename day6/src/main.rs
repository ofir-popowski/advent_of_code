use std::{fs, io};
use std::collections::HashMap;
use std::error::Error;

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("input.txt")?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_file()?;
    let groups = input.split("\n\n");
    let mut accumulator = 0;
    for group in groups {
        let mut answers: HashMap<char, i32> = HashMap::new();
        let mut people = 0;
        for person in group.lines() {
            people += 1;
            person.chars().for_each(|c| {
                if answers.contains_key(&c) {
                    let current_pair = &answers.get_key_value(&c).unwrap();
                    &answers.insert(current_pair.0.clone(), current_pair.1 + 1);
                } else {
                    &answers.insert(c, 1);
                }
            });
        }
        &answers.iter().for_each(|(_a, c)| {
            if c == &people {
                accumulator += 1;
            }
        });
    }
    println!("{}", accumulator);
    Ok(())
}
