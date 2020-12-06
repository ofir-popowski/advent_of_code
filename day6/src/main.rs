use std::{fs, io};
use std::collections::HashSet;
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
        let mut answers: HashSet<char> = HashSet::new();
        for person in group.lines() {
            person.chars().for_each(|c| {
                answers.insert(c);
            })
        }
        accumulator += answers.len();
    }
    println!("{}", accumulator);
    Ok(())
}
