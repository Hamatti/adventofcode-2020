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
    let mut forward: HashMap<String, Vec<String>> = HashMap::new();
    for line in input {
        let bag: (String, Vec<String>) = parse_line(line);
        let color: String = bag.0;
        let other_bags: Vec<String> = bag.1;
        &forward.insert(color, other_bags);
    }

    println!("{:?}", forward);
    Some(1)
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let re = Regex::new(r"^(\w+\s\w+) bags contain (?:(?:\d+ (\w+\s\w+) bags?(?:, |\.))+|no other bags)").unwrap();
    let sub_re = Regex::new(r"(?:\d (\w+\s\w+) bags?)+").unwrap();
    for cap in re.captures_iter(line) {
        let color: String = cap[1].to_string();
        let mut other_bags: Vec<String> = Vec::new();
        
        if line.contains("no other bags") {
            return (color, Vec::new());
        }

        let splitter = line.split(" bags contain ").collect::<Vec<&str>>()[1];
        for sub_cap in sub_re.captures_iter(splitter) {
            other_bags.push(sub_cap[1].to_string());
        }
        
        return (color, other_bags);
        
    }

    (String::new(), Vec::new())
 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_first_part() {
        let example_input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        assert_eq!(first_part(&example_input).unwrap(), 4);
    }

    #[test]
    fn it_parses_correctly() {
            assert_eq!(
                parse_line("light green bags contain 2 pale cyan bags.").0, "light green", 
            );
            assert_eq!(
                parse_line("light green bags contain 2 pale cyan bags.").1, vec!["pale cyan"] 
            );
            
            assert_eq!(
                parse_line("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.").0, "shiny gold", 
            );
            assert_eq!(
                parse_line("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.").1, vec!["dark olive", "vibrant plum"] 
            );

            assert_eq!(
                parse_line("dim tan bags contain 3 shiny teal bags, 5 bright white bags, 4 striped bronze bags.").0, "dim tan"
            );
            assert_eq!(
                parse_line("dim tan bags contain 3 shiny teal bags, 5 bright white bags, 4 striped bronze bags.").1, vec!["shiny teal", "bright white", "striped bronze"]
            );

            assert_eq!(
                parse_line("dull aqua bags contain no other bags.").0, "dull aqua"
            );

            assert_eq!(
                parse_line("dull aqua bags contain no other bags.").1.len() , 0
            );
    }
}