use std::collections::HashSet;
use std::error::Error;
use std::{fs, io};

fn read_file() -> Result<String, io::Error> {
    let contents = fs::read_to_string("passports.txt")?;
    Ok(contents)
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
        let fields = passport.split(" ");
        let mut existing_fields: HashSet<&str> = HashSet::new();
        for field in fields {
            let pairs = field.split(":");
            for pair in pairs.enumerate() {
                // print!("{:?}", pair);
                if pair.0 % 2 == 0 {
                    existing_fields.insert(pair.1);
                }
            }
        }
        let difference = mandatory_fields.difference(&existing_fields);
        if difference.count() == 0 {
            valid_passports += 1;
        }
    }
    println!("Valid passports: {}", valid_passports);
    Ok(())
}
