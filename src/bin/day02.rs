use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.split("\n").collect();
    let input2: Vec<&str> = input.clone();

    println!("Part 1: {:?}", validate_passwords(input));
    println!("Part 2: {:?}", validate_passwords_positional(input2));

    Ok(())

}

fn validate_passwords(input: Vec<&str>) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.*)$").unwrap();
    let mut valid: u32 = 0;
    for line in input {
        for cap in re.captures_iter(line) {
            let min: usize = cap[1].parse().unwrap();
            let max: usize = cap[2].parse().unwrap();
            let chr: &str = &cap[3];
            let password: &str = &cap[4];

            let counts: usize = password.matches(chr).count();

            if min <= counts && counts <= max {
                valid += 1;
            }
        }
    }
    valid
}

fn validate_passwords_positional(input: Vec<&str>) -> u32 {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(.*)$").unwrap();
    let mut valid: u32 = 0;
    for line in input {
        for cap in re.captures_iter(line) {
            let pos1: usize = cap[1].parse().unwrap();
            let pos2: usize = cap[2].parse().unwrap();
            let chr: &char = &cap[3].chars().nth(0).unwrap();
            let password = &mut cap[4].chars();

            let first_match = &password.clone().nth(pos1 - 1).unwrap();
            let second_match = &password.clone().nth(pos2 - 1).unwrap();

            if (first_match == chr && second_match != chr) || (first_match != chr && second_match == chr) {
                valid += 1;
            }
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let example_input = ["1-3 a: abcde","1-3 b: cdefg","2-9 c: ccccccccc"].to_vec();
        assert_eq!(validate_passwords(example_input), 2);
    }

    #[test]
    fn it_solves_part2_example() {
        let example_input = ["1-3 a: abcde","1-3 b: cdefg","2-9 c: ccccccccc"].to_vec();
        assert_eq!(validate_passwords_positional(example_input), 1);
    }
}
