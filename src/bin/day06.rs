use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let groups: Vec<Vec<&str>> = buffer
        .split("\n\n")
        .into_iter()
        .map(|groups| groups.split("\n").into_iter().collect())
        .collect();

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_runs_first_part() {
        let example_input = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];

        assert_eq!(first_part(&example_input).unwrap(), 11);
    }

    #[test]
    fn it_runs_second_part() {
        let example_input = vec![
            vec!["abc"],
            vec!["a", "b", "c"],
            vec!["ab", "ac"],
            vec!["a", "a", "a", "a"],
            vec!["b"],
        ];

        assert_eq!(second_part(&example_input).unwrap(), 6);
    }
}
