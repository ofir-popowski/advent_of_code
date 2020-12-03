use std::error::Error;
use std::fs::{self};
use std::io;

struct Terrain {
    content: Vec<Vec<char>>,
    current_line: usize,
    current_column: usize,
}

impl Terrain {
    fn new(content: String) -> Terrain {
        let mut terrain: Vec<Vec<char>> = Vec::new();
        for line in content.lines() {
            let chars: Vec<char> = line.chars().collect();
            terrain.push(chars);
        }
        Terrain {
            content: terrain,
            current_line: 0,
            current_column: 0,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        self.content.iter().for_each(|letter| {
            letter.iter().for_each(|letter| {
                print!("{}", letter);
            });
            println!();
        });
    }

    fn step(&mut self) -> char {
        self.current_line += 1;
        self.current_column += 3;
        if self.current_column >= 31 {
            self.current_column -= 31
        }
        let vec = &self.content[self.current_line];
        vec[self.current_column]
    }
}

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("map.txt")?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file()?;
    let mut terrain = Terrain::new(content);
    let mut counter = 0;
    for _ in 1..terrain.content.len() {
        let char = terrain.step();
        if char == '#' {
            counter += 1;
        }
    }
    // should be 268
    println!("Trees: {}", counter);
    Ok(())
}
