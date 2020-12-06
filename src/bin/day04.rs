use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.lines().collect();

    let mut passports: Vec<String> = vec![];
    let mut current_passport: String = String::new();

    for line in &input {
        if *line == "" {
            let pw: String = current_passport.clone();
            passports.push(pw);
            current_passport.clear();
        } else {
            current_passport.push_str(&line);
            current_passport.push_str(" ");
        }
    }
    passports.push(current_passport);

    println!("Part 1: {:?}", first_part(&passports).unwrap());
    println!("Part 2: {:?}", second_part(&passports).unwrap());
    Ok(())
}

fn first_part(passports: &Vec<String>) -> Option<u32> {
    let mut valid_passports: u32 = 0;

    for passport in passports {
        let parsed: HashMap<String, String> = parse_passport(&passport);
        if parsed.keys().len() == 8 || (parsed.keys().len() == 7 && !parsed.contains_key("cid")) {
            valid_passports += 1;
        }
    }

    Some(valid_passports)
}

fn parse_passport(passport: &String) -> HashMap<String, String> {
    let mut parsed: HashMap<String, String> = HashMap::new();
    let parts: Vec<&str> = passport.split(' ').collect::<Vec<&str>>();

    for part in &parts {
        if part.len() == 0 {
            continue;
        }
        let pparts: Vec<&str> = part.split(':').collect();
        let (key, value) = (pparts[0], pparts[1]);
        parsed.insert(String::from(key), String::from(value));
    }

    parsed
}

fn validate_field(key: &String, value: &String) -> bool {
    match &key[..] {
        "byr" => {
            let correct_length: bool = value.len() == 4;
            let value: usize = value.parse().unwrap();
            correct_length && value >= 1920 && value <= 2002
        }
        "iyr" => {
            let correct_length: bool = value.len() == 4;
            let value: usize = value.parse().unwrap();
            correct_length && value >= 2010 && value <= 2020
        }
        "eyr" => {
            let correct_length: bool = value.len() == 4;
            let value: usize = value.parse().unwrap();
            correct_length && value >= 2020 && value <= 2030
        }
        "hgt" => {
            if value.matches("cm").count() > 0 {
                let value: usize = value.replace("cm", "").parse().unwrap();
                return value >= 150 && value <= 193;
            }
            if value.matches("in").count() > 0 {
                let value: usize = value.replace("in", "").parse().unwrap();
                return value >= 59 && value <= 76;
            }
            false
        }
        "hcl" => {
            let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            re.is_match(value)
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()),
        "pid" => {
            let re = Regex::new(r"^\d{9}$").unwrap();
            re.is_match(value)
        }
        "cid" => true,
        _ => false,
    }
}

/// Passport is valid if:
/// - It has all 8 fields *OR* the only missing field is `cid`
/// - `byr` (Birth year) is four digits and between 1920 and 2002, inclusive
/// - `iyr` (Issue year) is four digits and between 2010 and 2020, inclusive
/// - `eyr` (Expiration year) is four digits and between 2020 and 2030, inclusive
/// - `hgt` (Height) is either between 150 and 193, inclusive and format `cm` *OR* between 59 and 76, inclusive and format `in`
/// - `hcl` (Hair color) is `#` followed by six characters, [0-9a-f]
/// - `ecl` (Eye color) is one of: amb, blu, brn, gry, grn, hzl, oth
/// - `pid` (Passport ID) is 9 digits, including leading zeroes
/// - `cid` (Country ID) is ignored, missing or not
fn second_part(passports: &Vec<String>) -> Option<u32> {
    let mut valid_passports: u32 = 0;

    for passport in passports {
        let parsed: HashMap<String, String> = parse_passport(passport);
        // Violation of rule 1
        if parsed.keys().len() < 7 {
            continue;
        }

        // Is invalid if there's 7 fields including `cid`
        if parsed.keys().len() == 7 {
            if parsed.contains_key("cid") {
                continue;
            }
        }

        let mut all_valid = true;
        for (k, v) in &parsed {
            if !validate_field(&k, &v) {
                all_valid = false;
                break;
            }
        }
        if all_valid {
            valid_passports += 1;
        }
    }

    Some(valid_passports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let example_input = vec![
            String::from(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
            ),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"),
            String::from("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"),
        ];
        assert_eq!(first_part(&example_input).unwrap(), 2);
    }

    #[test]
    fn it_solves_part2_example() {
        let example_input =
            vec![
            String::from("eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"), // INVALID, height is wrong
            String::from("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"), // INVALID eyr is wrong
            String::from("hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"), // INVALID
            String::from("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"), // INVALID

            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f"), // VALID
            String::from("eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"), // VALID
            String::from("hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022") // VALID
        ];
        assert_eq!(second_part(&example_input).unwrap(), 3);
    }

    #[test]
    fn it_validates_byr_correctly() {
        assert_eq!(
            validate_field(&String::from("byr"), &String::from("1919")),
            false
        );
        assert_eq!(
            validate_field(&String::from("byr"), &String::from("1920")),
            true
        );
        assert_eq!(
            validate_field(&String::from("byr"), &String::from("2002")),
            true
        );
        assert_eq!(
            validate_field(&String::from("byr"), &String::from("2003")),
            false
        );
    }

    #[test]
    fn it_validates_iyr_correctly() {
        assert_eq!(
            validate_field(&String::from("iyr"), &String::from("2009")),
            false
        );
        assert_eq!(
            validate_field(&String::from("iyr"), &String::from("2010")),
            true
        );
        assert_eq!(
            validate_field(&String::from("iyr"), &String::from("2020")),
            true
        );
        assert_eq!(
            validate_field(&String::from("iyr"), &String::from("2021")),
            false
        );
    }

    #[test]
    fn it_validates_eyr_correctly() {
        assert_eq!(
            validate_field(&String::from("eyr"), &String::from("2019")),
            false
        );
        assert_eq!(
            validate_field(&String::from("eyr"), &String::from("2020")),
            true
        );
        assert_eq!(
            validate_field(&String::from("eyr"), &String::from("2030")),
            true
        );
        assert_eq!(
            validate_field(&String::from("eyr"), &String::from("2031")),
            false
        );
    }

    #[test]
    fn it_validates_hgt_correctly() {
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("149cm")),
            false
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("150cm")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("193cm")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("194cm")),
            false
        );

        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("58in")),
            false
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("59in")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("76in")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("77in")),
            false
        );

        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("76")),
            false
        );
        assert_eq!(
            validate_field(&String::from("hgt"), &String::from("193")),
            false
        );
    }

    #[test]
    fn it_validates_hcl_correctly() {
        assert_eq!(
            validate_field(&String::from("hcl"), &String::from("ccc999")),
            false
        );
        assert_eq!(
            validate_field(&String::from("hcl"), &String::from("#ccc999")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hcl"), &String::from("#000000")),
            true
        );
        assert_eq!(
            validate_field(&String::from("hcl"), &String::from("#abvc")),
            false
        );
    }

    #[test]
    fn it_validates_ecl_correctly() {
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("amb")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("blu")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("brn")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("gry")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("grn")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("hzl")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("oth")),
            true
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("wrong")),
            false
        );
        assert_eq!(
            validate_field(&String::from("ecl"), &String::from("01234")),
            false
        );
    }

    #[test]
    fn it_validates_pid_correctly() {
        assert_eq!(
            validate_field(&String::from("pid"), &String::from("012345678")),
            true
        );
        assert_eq!(
            validate_field(&String::from("pid"), &String::from("1234")),
            false
        );
        assert_eq!(
            validate_field(&String::from("pid"), &String::from("01234567F")),
            false
        );
    }

    #[test]
    fn it_validates_cid_correctly() {
        assert_eq!(
            validate_field(&String::from("cid"), &String::from("012345678")),
            true
        );
        assert_eq!(
            validate_field(&String::from("cid"), &String::from("1234")),
            true
        );
        assert_eq!(
            validate_field(&String::from("cid"), &String::from("01234567F")),
            true
        );
        assert_eq!(
            validate_field(&String::from("cid"), &String::from("")),
            true
        );
    }
}
