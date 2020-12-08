use std::io::{self, Read};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.lines().collect();

    println!("First part: {}", first_part(&input).unwrap());
    Ok(())
}

fn first_part(input: &[&str]) -> Option<usize> {
    let mut dag = Dag::new();
    let mut all_bags: HashMap<String, Bag> = HashMap::new();
    let wanted_color: String = "shiny gold".to_string();
    let mut valid_bags: HashSet<String> = HashSet::new();

    for line in input {
        let bag: Bag = parse_line(line).unwrap();
        all_bags.insert((&*bag.color).to_string(), bag);
    }

    for color in all_bags.keys() {
        if directly_contains_color(&all_bags[color], &wanted_color) {
            valid_bags.insert(color.to_string());
        }
    }

    Some(valid_bags.len())
}

fn directly_contains_color(bag: &Bag, color: &str) -> bool {
    match &bag.other_bags {
        None => false,
        Some(bags) => bags.into_iter().any(|bag| bag == color)
    }
}

#[derive(Debug)]
struct Bag {
    color: String,
    other_bags: Option<Vec<String>>
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.other_bags.as_deref() == self.other_bags.as_deref()
    }
}

fn parse_line(line: &str) -> Option<Bag> {
    let re = Regex::new(r"^(\w+\s\w+) bags contain (?:(?:\d+ (\w+\s\w+) bags?(?:, |\.))+|no other bags)").unwrap();
    let sub_re = Regex::new(r"(?:\d (\w+\s\w+) bags)").unwrap();
    for cap in re.captures_iter(line) {
        let color: String = cap[1].to_string();
        let mut other_bags: Vec<String> = Vec::new();
        
        if line.contains("no other bags") {
            return Some(Bag {
                color,
                other_bags: None
            });
        }

        for sub_cap in sub_re.captures_iter(line.split(" bags contain ").collect::<Vec<&str>>()[1]) {
            other_bags.push(sub_cap[1].to_string());
        }
        
        return Some(Bag {
            color,
            other_bags: Some(other_bags)
        });
        
    }

    None
 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_correctly() {
            assert_eq!(
                parse_line("light green bags contain 2 pale cyan bags.").unwrap(),
                Bag { 
                    color: "light green".to_string(), 
                    other_bags: Some(vec!["pale cyan".to_string()])
                }
            );

            assert_eq!(
                parse_line("dim tan bags contain 3 shiny teal bags, 5 bright white bags, 4 striped bronze bags.").unwrap(),
                Bag { 
                    color: "dim tan".to_string(), 
                    other_bags: Some(vec!["shiny teal".to_string(), "bright white".to_string(), "striped bronze".to_string()])
                }
            );

            assert_eq!(
                parse_line("dull aqua bags contain no other bags.").unwrap(), 
                Bag {
                    color: "dull aqua".to_string(),
                    other_bags: None
                }
            );


    }
}