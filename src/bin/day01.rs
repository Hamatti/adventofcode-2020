use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<i32> = buffer
        .split("\n")
        .filter(|inp| inp != &"")
        .map(|num| num.parse().unwrap())
        .collect();

    let input2: Vec<i32> = input.clone();

    match find_double(input) {
        Some(result) => println!("Part 1: {}", result),
        None => panic!("Oh no!"),
    }

    match find_triplet(input2) {
        Some(result) => println!("Part 2: {}", result),
        None => panic!("Oh no!"),
    }

    Ok(())
}

fn find_double(input: Vec<i32>) -> Option<i32> {
    for num1 in &input {
        for num2 in &input {
            if num1 + num2 == 2020 {
                return Some(num1 * num2);
            }
        }
    }
    None
}

fn find_triplet(input: Vec<i32>) -> Option<i32> {
    for num1 in &input {
        for num2 in &input {
            for num3 in &input {
                if num1 + num2 + num3 == 2020 {
                    return Some(num1 * num2 * num3);
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let example_input = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(find_double(example_input).unwrap(), 514579);
    }

    #[test]
    fn it_solves_part2_example() {
        let example_input = [1721, 979, 366, 299, 675, 1456].to_vec();
        assert_eq!(find_triplet(example_input).unwrap(), 241861950);
    }
}
