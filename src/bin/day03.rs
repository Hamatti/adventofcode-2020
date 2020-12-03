use std::io::{self, Read};

const TREE: char = '#';

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let input: Vec<&str> = buffer.lines().collect();

    println!("Part 1: {:?}", first_part(&input).unwrap());
    println!(
        "Part 2: {:?}",
        second_part(&input, &[
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2)
        ]).unwrap()
    );

    Ok(())
}

#[derive(Copy, Clone)]
struct Slope {
    delta_row: usize,
    delta_col: usize
}

impl Slope {
    fn new(delta_col: usize, delta_row: usize) -> Slope {
        Slope {
            delta_row,
            delta_col
        }
    }
}

/// Calculates how many # characters are in the line of movement
/// When advancing in the 2D matrix by the rule of
/// `slope.1` row forward, `slope.0` columns forward
/// 
/// The input field will repeat to infinity to the right with copying
/// the pattern before it:
/// 
/// '#..#..#....#' is same as
/// '#..#..#....##..#..#....##..#..#....##..#..#....##..#..#....##..#..#....#' to infinity
fn solve_slope(input: &[&str], slope: Slope) -> Option<usize> {
    let mut row: usize = 0;
    let mut col: usize = 0;
    
    let max_rows: usize = input.len();
    let max_cols: usize = input[0].len();

    let mut trees: usize = 0;

    loop {
        row += slope.delta_row;
        col += slope.delta_col;

        if row >= max_rows {
            break;
        }

        // The input repeats to infinity in columns with the same pattern
        // so we need to loop back to beginning once we go over
        if col >= max_cols {
            col -= max_cols;
        }

        let chr = input[row].chars().nth(col)?;

        if chr == TREE {
            trees += 1;
        }
    }
    Some(trees)
}

/// Solve the slope for single slope of 3, 1
fn first_part(input: &[&str]) -> Option<usize> {
    solve_slope(&input, Slope::new(3, 1))
}

/// The product of slope difficulties for multiple slopes
fn second_part(input: &[&str], slopes: &[Slope]) -> Option<usize> {
    let mut slope_difficulties = vec![];
    for slope in slopes {
        slope_difficulties.push(solve_slope(&input, *slope).unwrap());
    }

    let mut slope_difficulty: usize = 1;
    for difficulty in slope_difficulties {
        slope_difficulty *= difficulty;
    }

    Some(slope_difficulty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_part1_example() {
        let example_input = [
            "..##.........##.........##.........##.........##.........##.......",
            "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
            ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
            "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
            ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
            "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
            ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
            ".#........#.#........#.#........#.#........#.#........#.#........#",
            "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
            "#...##....##...##....##...##....##...##....##...##....##...##....#",
            ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#",
        ];
        assert_eq!(first_part(&example_input).unwrap(), 7);
    }

    #[test]
    fn it_solves_part2_example() {
        let example_input = [
            "..##.........##.........##.........##.........##.........##.......",
            "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..",
            ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.",
            "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#",
            ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.",
            "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....",
            ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#",
            ".#........#.#........#.#........#.#........#.#........#.#........#",
            "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...",
            "#...##....##...##....##...##....##...##....##...##....##...##....#",
            ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#",
        ].to_vec();
        let example_slopes = [
            Slope::new(1, 1),
            Slope::new(3, 1),
            Slope::new(5, 1),
            Slope::new(7, 1),
            Slope::new(1, 2)
        ];
        assert_eq!(second_part(&example_input, &example_slopes).unwrap(), 336);
    }
}
