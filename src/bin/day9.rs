use std::collections::VecDeque;

fn main() {
    let input: &str = include_str!("../../inputs/day9.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(compute1)
        .sum()
}

fn compute1(nums: Vec<i64>) -> i64 {
    dbg!(&nums);
    let mut ready = false;
    let mut history = VecDeque::new();
    history.push_front(nums.clone());
    let mut diff = vec![];
    let mut curr = nums.clone();
    while !ready {
        for i in 1..curr.len() {
            diff.push(curr[i] - curr[i - 1]);
        }
        ready = diff.iter().all(|&x| x == 0);
        if !ready {
            history.push_front(diff.clone());
            curr = diff.clone();
            diff = vec![];
        }
    }

    dbg!(&history);

    let mut row = history.pop_front().unwrap();
    let mut to_add = row.last().unwrap();
    let mut val = 0 + to_add;
    dbg!(&val, &to_add);

    while !history.is_empty() {
        row = history.pop_front().unwrap();
        to_add = row.last().unwrap();
        val += to_add;
    }

    dbg!(&val);

    val
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(compute2)
        .sum()
}

fn compute2(nums: Vec<i64>) -> i64 {
    dbg!(&nums);
    let mut ready = false;
    let mut history = VecDeque::new();
    history.push_front(nums.clone());
    let mut diff = vec![];
    let mut curr = nums.clone();
    while !ready {
        for i in 1..curr.len() {
            diff.push(curr[i] - curr[i - 1]);
        }
        ready = diff.iter().all(|&x| x == 0);
        if !ready {
            history.push_front(diff.clone());
            curr = diff.clone();
            diff = vec![];
        }
    }

    dbg!(&history);

    let mut row = history.pop_front().unwrap();
    let mut val = row[0];
    let mut to_subtract = val;
    dbg!(&val, &to_subtract);

    while !history.is_empty() {
        row = history.pop_front().unwrap();
        val = row[0] - to_subtract;
        to_subtract = val;
        dbg!(&val, &to_subtract);
    }

    dbg!(&val);

    val
}

#[test]
fn example() {
    let example: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part1(example), 114);
    assert_eq!(part2(example), 2);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day9.txt");
    assert_eq!(part1(input), 1842168671);
    assert_eq!(part2(input), 903);
}
