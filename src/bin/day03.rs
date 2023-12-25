use std::collections::VecDeque;

fn main() {
    let input: &str = include_str!("../../inputs/day03.txt");
    println!(
        "Part1: {}",
        solve_with(input, &part1_symbol_predicate, &part1_agg_fn)
    );
    println!(
        "Part2: {}",
        solve_with(input, &part2_symbol_predicate, &part2_agg_fn)
    );
}

fn part1_symbol_predicate(c: &char) -> bool {
    !c.is_ascii_digit() && *c != '.'
}

fn part1_agg_fn(numbers: Vec<u32>) -> Option<u32> {
    Some(numbers.iter().sum())
}

fn part2_symbol_predicate(c: &char) -> bool {
    *c == '*'
}

fn part2_agg_fn(numbers: Vec<u32>) -> Option<u32> {
    if numbers.len() == 2 {
        Some(numbers[0] * numbers[1])
    } else {
        None
    }
}

fn solve_with(
    input: &str,
    symbol_pred: &impl Fn(&char) -> bool,
    agg_fn: &impl Fn(Vec<u32>) -> Option<u32>,
) -> u32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let n: usize = grid.len();
    let m: usize = grid[0].len();
    let mut seen = vec![vec![false; m]; n];

    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, elem)| match elem {
                    c if symbol_pred(c) => Some((i, j)),
                    _ => None,
                })
        })
        .filter_map(|(i, j)| {
            let mut numbers = Vec::new();
            for (r, c) in adjacent(i, j, n, m) {
                if !seen[r][c] && grid[r][c].is_ascii_digit() {
                    let mut digits: VecDeque<_> =
                        VecDeque::from([grid[r][c].to_digit(10).unwrap()]);
                    seen[r][c] = true;

                    // check left
                    let mut left = 1;
                    while c >= left && grid[r][c - left].is_ascii_digit() {
                        digits.push_front(grid[r][c - left].to_digit(10).unwrap());
                        seen[r][c - left] = true;
                        left += 1;
                    }

                    // check right
                    let mut right = 1;
                    while c + right < m && grid[r][c + right].is_ascii_digit() {
                        digits.push_back(grid[r][c + right].to_digit(10).unwrap());
                        seen[r][c + right] = true;
                        right += 1;
                    }

                    numbers.push(digits.iter().fold(0, |acc, x| acc * 10 + x));
                }
            }
            agg_fn(numbers)
        })
        .sum()
}

fn adjacent(row: usize, col: usize, n: usize, m: usize) -> Vec<(usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        // (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter_map(|(r, c)| {
        let r = row as isize + r;
        let c = col as isize + c;
        if r >= 0 && r < n as isize && c >= 0 && c < m as isize {
            Some((r as usize, c as usize))
        } else {
            None
        }
    })
    .collect()
}

#[test]
fn example() {
    let example: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(
        solve_with(example, &part1_symbol_predicate, &part1_agg_fn),
        4361
    );
    assert_eq!(
        solve_with(example, &part2_symbol_predicate, &part2_agg_fn),
        467835
    );
}

#[test]
/// for refractoring
fn answer() {
    let input: &str = include_str!("../../inputs/day03.txt");
    assert_eq!(
        solve_with(input, &part1_symbol_predicate, &part1_agg_fn),
        525911
    );
    assert_eq!(
        solve_with(input, &part2_symbol_predicate, &part2_agg_fn),
        75805607
    );
}
