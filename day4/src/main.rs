use std::collections::HashSet;
use std::error::Error;
use std::{fs, io};

use validations::validations::is_field_valid;

mod validations;

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("passports.txt")?;
    Ok(contents)
}

fn check_field_names(passport: &str, mandatory_fields: &HashSet<&str>) -> bool {
    let fields = passport.split(" ");
    let mut existing_fields: HashSet<&str> = HashSet::new();
    for field in fields {
        let pairs = field.split(":");
        for pair in pairs.enumerate() {
            if pair.0 % 2 == 0 {
                existing_fields.insert(pair.1);
            }
        }
    }
    let difference = mandatory_fields.difference(&existing_fields);
    difference.count() == 0
}

fn check_field_values(passport: &str) -> bool {
    let fields = passport.split(" ");
    let mut valid_passport = true;
    for field in fields {
        let split: Vec<&str> = field.split(":").collect();
        let pair: (&str, &str) = (split[0], split[1]);
        if !is_field_valid(pair.0, pair.1) {
            valid_passport = false;
        }
    }
    valid_passport
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file()?;
    let content = content.split("\n\n");
    let mut mandatory_fields: HashSet<&str> = HashSet::new();
    for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        mandatory_fields.insert(field);
    }

    let mut valid_passports = 0;

    for passport_str in content {
        let passport = passport_str.replace("\n", " ");
        if check_field_names(&passport, &mandatory_fields) && check_field_values(&passport) {
            valid_passports += 1;
        }
    }
    println!("Valid passports: {}", valid_passports);
    Ok(())
}
