use std::error::Error;
use std::fs::{self};
use std::io;

struct Terrain {
    content: Vec<Vec<char>>,
    current_line: usize,
    current_column: usize
}

impl Terrain {
    fn new(content: &String) -> Terrain {
        let mut terrain: Vec<Vec<char>> = Vec::new();
        for line in content.lines() {
            let chars: Vec<char> = line.chars().collect();
            terrain.push(chars);
        }
        Terrain {
            content: terrain,
            current_line: 0,
            current_column: 0
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

    fn step(&mut self, slope: (usize, usize)) -> char {
        self.current_line += slope.1;
        self.current_column += slope.0;
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
    /*
    Right 1, down 1.
    Right 3, down 1.
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */

    let content = read_file()?;
    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];
    let mut total: i64 = 1;
    for &slope in slopes.iter() {
        let mut terrain = Terrain::new(&content);
        let mut counter = 0;
        for _ in 0..terrain.content.len() - 1 {
            if terrain.current_line + slope.1 > 323 {
                break
            }
            let char = terrain.step(slope);
            if char == '#' {
                counter += 1;
            }
        }
        println!("Slope: ({}, {}), Trees: {}", slope.0, slope.1 , counter);
        total *= counter;
    }
    println!("Total: {}", total);
    Ok(())
}
