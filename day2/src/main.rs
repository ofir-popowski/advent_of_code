use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{BitXor, RangeInclusive};

trait Password {
    fn new(password: String) -> Self;
    fn validate(&self) -> bool;
}

#[derive(Debug)]
struct PartOnePassword {
    pub range: RangeInclusive<i32>,
    pub char: char,
    pub password: String,
}

impl Password for PartOnePassword {
    fn new(line: String) -> PartOnePassword {
        let mut split_password = line.split(' ');

        let range = split_password.next().unwrap().to_string();
        let mut range = range.split('-');
        let lower_bound = range.next().unwrap().parse().unwrap();
        let upper_bound = range.next().unwrap().parse().unwrap();
        let range = lower_bound..=upper_bound;

        let char = split_password.next().unwrap().to_string();
        let char = char.chars().next().unwrap();

        let string = split_password.next().unwrap();

        PartOnePassword {
            range,
            char,
            password: string.to_string(),
        }
    }

    fn validate(&self) -> bool {
        let char_occurrences: i32 = self.password.matches(self.char).count() as i32;
        self.range.contains(char_occurrences.borrow())
    }
}

#[derive(Debug)]
struct PartTwoPassword {
    first_position: usize,
    second_position: usize,
    char: char,
    password: String,
}

impl Password for PartTwoPassword {
    fn new(line: String) -> PartTwoPassword {
        let mut split_password = line.split(' ');

        let range = split_password.next().unwrap().to_string();
        let mut range = range.split('-');
        let first_position = range.next().unwrap().parse().unwrap();
        let second_position = range.next().unwrap().parse().unwrap();

        let char = split_password.next().unwrap().to_string();
        let char = char.chars().next().unwrap();

        let string = split_password.next().unwrap();

        PartTwoPassword {
            first_position,
            second_position,
            char,
            password: string.to_string(),
        }
    }

    fn validate(&self) -> bool {
        let positions: Vec<char> = self.password.chars().collect();
        (positions[self.first_position - 1] == self.char)
            .bitxor(positions[self.second_position - 1] == self.char)
    }
}

fn main() {
    let passwords_file = File::open("passwords.txt").unwrap();
    let buff_reader = BufReader::new(passwords_file);
    let mut part_one_counter = 0;
    let mut part_two_counter = 0;

    for line in buff_reader.lines() {
        let line = line.unwrap();

        let part_one_password = PartOnePassword::new(line.clone());
        if part_one_password.validate() {
            part_one_counter += 1;
        }

        let part_two_password = PartTwoPassword::new(line.clone());
        if part_two_password.validate() {
            part_two_counter += 1;
        }
    }

    println!("Valid part one passwords: {}", part_one_counter);
    println!("Valid part two passwords: {}", part_two_counter);
}
