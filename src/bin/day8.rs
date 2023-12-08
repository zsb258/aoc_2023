use num::integer::lcm;
use std::collections::HashMap;

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let splits = input.split_once("\n\n");

    let instructions = splits.unwrap().0.chars().collect::<Vec<char>>();

    let nodes = splits
        .unwrap()
        .1
        .lines()
        .map(|line| {
            let splits = line.split_once(" = ");
            let name = splits.unwrap().0;
            let children = match splits
                .unwrap()
                .1
                .split_once('(')
                .unwrap()
                .1
                .split(')')
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

    (instructions, nodes)
}

fn traverse(
    instructions: &[char],
    nodes: &HashMap<&str, (&str, &str)>,
    start: &str,
    stop_pred: Box<dyn Fn(&str) -> bool>,
) -> u64 {
    let mut curr = start;
    let mut count = 0;
    let mut i = 0;

    while !stop_pred(curr) {
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
}

fn main() {
    let input: &str = include_str!("../../inputs/day8.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let (instructions, nodes) = parse(input);

    traverse(&instructions, &nodes, "AAA", Box::new(|name| name == "ZZZ"))
}

fn part2(input: &str) -> u64 {
    let (instructions, nodes) = parse(input);

    nodes
        .iter()
        .filter_map(|(name, _)| {
            if name.ends_with('A') {
                Some(traverse(
                    &instructions,
                    &nodes,
                    name,
                    Box::new(|name| name.ends_with('Z')),
                ))
            } else {
                None
            }
        })
        .fold(1, lcm)
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
