use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Password(RangeInclusive<i32>, char, String);

impl Password {
    fn new(line: String) -> Password {
        let mut split_password = line.split(' ');

        let range = split_password.next().unwrap().to_string();
        let mut range = range.split('-');
        let lower_bound = range.next().unwrap();
        let lower_bound: i32 = lower_bound.parse().unwrap();
        let upper_bound = range.next().unwrap();
        let upper_bound: i32 = upper_bound.parse().unwrap();
        let range = lower_bound..=upper_bound;

        let char = split_password.next().unwrap().to_string();
        let char = char.chars().next().unwrap();

        let string = split_password.next().unwrap();

        Password(range, char, string.to_string())
    }
}

fn main() {
    let passwords_file = File::open("passwords.txt").unwrap();
    let buff_reader = BufReader::new(passwords_file);
    let mut counter = 0;

    for line in buff_reader.lines() {
        let password = Password::new(line.unwrap());
        let char_occurrences: &i32 = &(password.2.matches(password.1).count() as i32);
        if password.0.contains(char_occurrences) {
            counter += 1;
        }
    }

    println!("Valid passwords: {}", counter);
}
