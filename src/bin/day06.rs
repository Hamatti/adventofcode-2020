use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.split("\n\n").collect();

    let mut groups: Vec<Vec<&str>> = Vec::new();

    for group in &input {
        groups.push(group.split("\n").collect());
    }

    println!("Part 1: {:?}", first_part(&groups).unwrap());
    println!("Part 2: {:?}", second_part(&groups).unwrap());
    Ok(())
}

fn first_part(groups: &Vec<Vec<&str>>) -> Option<usize> {
    let mut sum: usize = 0;
    for group in groups {
        let combined: String = group.join("");
        let mut unique = HashSet::new();
        for chr in combined.chars() {
            unique.insert(chr);
        }
        sum += unique.len();
    }

    Some(sum)
}

fn second_part(groups: &Vec<Vec<&str>>) -> Option<usize> {
    let mut sum: usize = 0;
    for group in groups {
        let mut sets: Vec<HashSet<char>> = vec![];
        for answers in group {
            let mut set = HashSet::new();
            for chr in answers.chars() {
                set.insert(chr);
            }
            sets.push(set);
        }

        let set1 = &sets[0];
        let intersection: Vec<&char> = set1
            .iter()
            .filter(|k| sets.clone().into_iter().all(|s| s.contains(k)))
            .collect();

        sum += intersection.len()
    }

    Some(sum)
}
