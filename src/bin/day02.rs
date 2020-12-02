use regex::Regex;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.split('\n').collect();

    println!("Part 1: {:?}", validate_passwords(&input).unwrap());
    println!(
        "Part 2: {:?}",
        validate_passwords_positional(&input).unwrap()
    );

    Ok(())
}

fn validate_passwords(input: &Vec<&str>) -> Option<u32> {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.*)$").unwrap();
    let mut valid: u32 = 0;
    for line in input {
        for cap in re.captures_iter(line) {
            let min: usize = cap[1].parse().ok()?;
            let max: usize = cap[2].parse().ok()?;
            let chr: &str = &cap[3];
            let password: &str = &cap[4];

            if is_valid_password_old(password, chr, min, max) {
                valid += 1;
            }
        }
    }
    Some(valid)
}

/// Checks if a given password matches its requirements
///
/// Requirements:
/// the amount of `inspection_character` in the password must be between `min` and `max`, inclusive
fn is_valid_password_old(
    password: &str,
    inspection_character: &str,
    min: usize,
    max: usize,
) -> bool {
    let valid_characters: usize = password.matches(inspection_character).count();

    min <= valid_characters && valid_characters <= max
}

fn validate_passwords_positional(input: &Vec<&str>) -> Option<u32> {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.*)$").unwrap();
    let mut valid: u32 = 0;
    for line in input {
        for cap in re.captures_iter(line) {
            let pos1: usize = cap[1].parse().ok()?;
            let pos2: usize = cap[2].parse().ok()?;
            let chr: &char = &cap[3].chars().next()?;
            let password = &mut cap[4].chars();

            let first_match = &password.clone().nth(pos1 - 1)?;
            let second_match = &password.clone().nth(pos2 - 1)?;

            if (first_match == chr && second_match != chr)
                || (first_match != chr && second_match == chr)
            {
                valid += 1;
            }
        }
    }

    Some(valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let example_input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].to_vec();
        assert_eq!(validate_passwords(&example_input).unwrap(), 2);
    }

    #[test]
    fn it_solves_part2_example() {
        let example_input = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].to_vec();
        assert_eq!(validate_passwords_positional(&example_input).unwrap(), 1);
    }
}
