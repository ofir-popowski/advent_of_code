pub mod validations {
    use regex::Regex;

    fn validate_byr(val: &str) -> bool {
        match val.parse::<i32>() {
            Ok(num) => (1920..=2002).contains(&num),
            Err(_) => false,
        }
    }

    fn validate_iyr(val: &str) -> bool {
        match val.parse::<i32>() {
            Ok(num) => (2010..=2020).contains(&num),
            Err(_) => false,
        }
    }

    fn validate_eyr(val: &str) -> bool {
        match val.parse::<i32>() {
            Ok(num) => (2020..=2030).contains(&num),
            Err(_) => false,
        }
    }

    fn validate_hgt(val: &str) -> bool {
        let pattern = Regex::new("^([0-9]+)(cm|in)$").unwrap();
        let captures = pattern.captures(val);
        match captures {
            None => false,
            Some(cap) => {
                let height = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let measure = cap.get(2).unwrap().as_str();
                let valid = match measure {
                    "cm" => (150..=193).contains(&height),
                    "in" => (59..=76).contains(&height),
                    _ => false,
                };
                valid
            }
        }
    }

    fn validate_hcl(val: &str) -> bool {
        let pattern = Regex::new("^#[0-9a-f]{6}$").unwrap();
        pattern.is_match(val)
    }

    fn validate_ecl(val: &str) -> bool {
        match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn validate_pid(val: &str) -> bool {
        let pattern = Regex::new("^[0-9]{9}$").unwrap();
        pattern.is_match(val)
    }

    #[allow(unused_variables)]
    fn validate_cid(val: &str) -> bool {
        true
    }

    pub fn is_field_valid(field_name: &str, val: &str) -> bool {
        match field_name {
            "byr" => validate_byr(val),
            "iyr" => validate_iyr(val),
            "eyr" => validate_eyr(val),
            "hgt" => validate_hgt(val),
            "hcl" => validate_hcl(val),
            "ecl" => validate_ecl(val),
            "pid" => validate_pid(val),
            "cid" => validate_cid(val),
            _ => panic!("Error"),
        }
    }
}
