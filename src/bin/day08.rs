use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<String> = buffer.lines().into_iter().map(|inp| String::from(inp)).collect();

    println!("Part 1: {:?}", first_part(&input).unwrap());
    println!("Part 2: {:?}", second_part(&input).unwrap());
    Ok(())
}

fn first_part(input: &Vec<String>) -> Option<i64> {
    let mut cur_idx: i64 = 0;
    let mut accumulator: i64 = 0;
    let mut visited_idxs: HashSet<i64> = HashSet::new();

    loop {
        let parts: Vec<&str> = input[cur_idx as usize].split(' ').collect();
        
        let (operation, value) = (parts[0], parts[1]);
        let value: i64 = value.parse().unwrap();

        match operation {
            "nop" => {
                cur_idx += 1;
            }
            "acc" => {
                accumulator += value;
                cur_idx += 1;
            }
            "jmp" => {
                cur_idx += value;
            }
            _ => { continue; }
        }


        if visited_idxs.contains(&cur_idx) {
            return Some(accumulator);
        }


        visited_idxs.insert(cur_idx);
    }
}

fn second_part(original_input: &Vec<String>) -> Option<i64> {
    let mut last_checked_idx: usize = 0;
    let mut input = original_input.clone();
    let mut result: Option<i64>;
    let mut replace_from: String = String::from("jmp");
    let mut replace_to: String = String::from("nop");
    loop {
        result = run_program(&input);
        match result {
            Some(_) => {break;},
            None => {
                let found = input[last_checked_idx+1..].iter().position(|operation| operation.starts_with(&replace_from));
                if found == None {
                    replace_from = String::from("nop");
                    replace_to = String::from("jmp");
                    last_checked_idx = 0;
                }
                last_checked_idx = found.unwrap() + last_checked_idx + 1;
                input = original_input.clone();
                input[last_checked_idx] = input[last_checked_idx as usize].replace(&replace_from, &replace_to);
            }
        }
    }

    result

}

fn run_program(program: &Vec<String>) -> Option<i64> {
    let mut cur_idx: i64 = 0;
    let mut accumulator: i64 = 0;
    let mut visited_idxs: HashSet<i64> = HashSet::new();

    loop {
        let parts: Vec<&str> = program[cur_idx as usize].split(' ').collect();
        
        let (operation, value) = (parts[0], parts[1]);
        let value: i64 = value.parse().unwrap();

        
        match operation {
            "nop" => {
                cur_idx += 1;
            }
            "acc" => {
                accumulator += value;
                cur_idx += 1;
            }
            "jmp" => {
                cur_idx += value;
            }
            _ => { continue; }
        }

        // If program loops over itself, it's an invalid program
        if visited_idxs.contains(&cur_idx) {
            return None;
        }

        // If we reach the end of the program, it's a valid program
        if cur_idx == program.len() as i64 {
            break;
        }

        visited_idxs.insert(cur_idx);
    }
    Some(accumulator)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_first_part() {
        let example_input = vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ];

        assert_eq!(first_part(&example_input).unwrap(), 5);
    }

    #[test]
    fn it_runs_second_part() {
        let example_input = vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ];

        assert_eq!(second_part(&example_input).unwrap(), 8);
    }
   
}
