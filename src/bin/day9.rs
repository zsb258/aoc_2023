fn main() {
    let input: &str = include_str!("../../inputs/day9.txt");
    println!("Part1: {}", solve_with(input, &part1_compute));
    println!("Part2: {}", solve_with(input, &part2_compute));
}

fn solve_with(input: &str, solver: &dyn Fn(Vec<Vec<i64>>) -> i64) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|nums| {
            let mut history = vec![nums]; // using as stack
            loop {
                let curr = history.last().unwrap();
                let diff = (1..curr.len())
                    .map(|i| curr[i] - curr[i - 1])
                    .collect::<Vec<_>>();
                if diff.iter().any(|&x| x != 0) {
                    history.push(diff);
                } else {
                    break history;
                }
            }
        })
        .map(solver)
        .sum()
}

fn part1_compute(history: Vec<Vec<i64>>) -> i64 {
    history
        .iter()
        .rev()
        .fold(0, |acc, v| acc + v.last().unwrap())
}

fn part2_compute(history: Vec<Vec<i64>>) -> i64 {
    history.iter().rev().fold(0, |acc, v| v[0] - acc)
}

#[test]
fn example() {
    let example: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(solve_with(example, &part1_compute), 114);
    assert_eq!(solve_with(example, &part2_compute), 2);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day9.txt");
    assert_eq!(solve_with(input, &part1_compute), 1842168671);
    assert_eq!(solve_with(input, &part2_compute), 903);
}
