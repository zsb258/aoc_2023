use num::integer::lcm;
use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../../inputs/day8.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut splits = input.splitn(2, "\n\n");
    let instructions = splits.next().unwrap().chars().collect::<Vec<char>>();
    let nodes = splits
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let name = line.split(" = ").next().unwrap();
            let children = match line
                .split(" = ")
                .nth(1)
                .unwrap()
                .splitn(2, '(')
                .nth(1)
                .unwrap()
                .splitn(2, ')')
                .next()
                .unwrap()
                .splitn(2, ", ")
                .collect::<Vec<_>>()[..]
            {
                [lt, rt] => (lt, rt),
                _ => panic!("Invalid input"),
            };
            (name, children)
        })
        .fold(HashMap::new(), |mut acc, (name, children)| {
            acc.insert(name, children);
            acc
        });

    let mut curr = "AAA";
    let mut count = 0;
    let mut i = 0;

    while curr != "ZZZ" {
        // println!("{}", curr);
        curr = match instructions.get(i) {
            Some('L') => nodes.get(curr).unwrap().0,
            Some('R') => nodes.get(curr).unwrap().1,
            _ => panic!("Invalid input"),
        };
        // println!("{:?}", curr);
        if i == instructions.len() - 1 {
            i = 0;
        } else {
            i += 1;
        }
        count += 1;
    }

    count
}

fn part2(input: &str) -> u64 {
    let mut splits = input.splitn(2, "\n\n");
    let instructions = splits.next().unwrap().chars().collect::<Vec<char>>();
    let nodes = splits
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let name = line.split(" = ").next().unwrap();
            let children = match line
                .split(" = ")
                .nth(1)
                .unwrap()
                .splitn(2, '(')
                .nth(1)
                .unwrap()
                .splitn(2, ')')
                .next()
                .unwrap()
                .splitn(2, ", ")
                .collect::<Vec<_>>()[..]
            {
                [lt, rt] => (lt, rt),
                _ => panic!("Invalid input"),
            };
            (name, children)
        })
        .fold(HashMap::new(), |mut acc, (name, children)| {
            acc.insert(name, children);
            acc
        });

    let curr_nodes = nodes
        .iter()
        .filter_map(|(name, _)| {
            if name.ends_with('A') {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let steps = curr_nodes.iter().map(|name| {
        let mut curr = name.as_str();
        let mut count = 0;
        let mut i = 0;
        while !curr.ends_with('Z') {
            curr = match instructions.get(i) {
                Some('L') => nodes.get(curr).unwrap().0,
                Some('R') => nodes.get(curr).unwrap().1,
                _ => panic!("Invalid input"),
            };
            if i == instructions.len() - 1 {
                i = 0;
            } else {
                i += 1;
            }
            count += 1;
        }
        count
    });

    steps.fold(1, |acc, x| lcm(acc, x as u64))
}

#[test]
fn example_part1() {
    let example1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(example1), 2);

    let example2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(example2), 6);
}

#[test]
fn example_part2() {
    let example: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(part2(example), 6);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day8.txt");
    assert_eq!(part1(input), 16897);
    assert_eq!(part2(input), 16563603485021);
}
