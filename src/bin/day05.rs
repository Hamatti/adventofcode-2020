use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.lines().collect();

    println!("Part 1: {:?}", first_part(&input).unwrap());
    println!("Part 2: {:?}", second_part(&input).unwrap());
    Ok(())
}

fn first_part(input: &Vec<&str>) -> Option<u32> {
    let mut max_seat_id = 0;
    for boarding_pass in input {
        let seat_id = calculate_seat_id(boarding_pass);
        if seat_id > max_seat_id {
            max_seat_id = seat_id
        }
    }

    Some(max_seat_id)
}

fn second_part(input: &Vec<&str>) -> Option<u32> {
    let mut all_seats: Vec<u32> = Vec::new();
    for boarding_pass in input {
        let seat_id = calculate_seat_id(boarding_pass);
        all_seats.push(seat_id);
    }
    all_seats.sort();

    let my_seat_id: Option<u32> = find_my_seat_id(&all_seats);
    my_seat_id
}

fn find_my_seat_id(seat_ids: &Vec<u32>) -> Option<u32> {
    for idx in 0..(seat_ids.len() - 1) {
        if !(seat_ids[idx] == seat_ids[idx + 1] - 1) {
            return Some(seat_ids[idx] + 1);
        }
    }
    None
}

fn calculate_seat_id(boarding_pass: &str) -> u32 {
    let mut idx: u32 = 0;

    let mut min_row: u32 = 0;
    let mut max_row: u32 = 127;
    let mut min_col: u32 = 0;
    let mut max_col: u32 = 7;

    for chr in boarding_pass.chars() {
        if idx < 7 {
            let res: (u32, u32) = helper(chr, min_row, max_row);
            min_row = res.0;
            max_row = res.1;
        } else {
            let res: (u32, u32) = helper(chr, min_col, max_col);
            min_col = res.0;
            max_col = res.1;
        }

        idx += 1;
    }

    let seat_id = max_row * 8 + max_col;

    seat_id
}

fn helper(direction: char, min: u32, max: u32) -> (u32, u32) {
    let half: u32 = max - (max - min) / 2;
    if direction == 'B' || direction == 'R' {
        (half, max)
    } else {
        (min, half - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_runs_first_part_right() {
        let example_input: Vec<&str> = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"];

        assert_eq!(first_part(&example_input).unwrap(), 820);
    }

    #[test]
    fn it_runs_second_part_right() {
        let example_input: Vec<&str> = vec!["FFFBBBFRRR", "FFFBBBFRRL", "FFFBBBFRLL"];

        assert_eq!(second_part(&example_input).unwrap(), 117);
    }

    #[test]
    fn it_calculates_id_correctly() {
        assert_eq!(calculate_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(calculate_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(calculate_seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn it_helpers_correctly() {
        assert_eq!(helper('F', 0, 128), (0, 63));
        assert_eq!(helper('B', 0, 63), (32, 63));
        assert_eq!(helper('F', 32, 63), (32, 47));
        assert_eq!(helper('B', 32, 47), (40, 47));
        assert_eq!(helper('B', 40, 47), (44, 47));
        assert_eq!(helper('F', 44, 47), (44, 45));
        assert_eq!(helper('F', 44, 45), (44, 44));
    }
}
