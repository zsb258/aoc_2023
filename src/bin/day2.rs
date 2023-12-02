use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/day2.txt");
// const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

const MAX_R: i32 = 12;
const MAX_G: i32 = 13;
const MAX_B: i32 = 14;

fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

fn part1() -> i32 {
    INPUT
        .lines()
        .filter_map(|line| {
            let game = parse_line(line);
            let r = game
                .draws
                .iter()
                .map(|draw| draw.get(&'r').unwrap_or(&0))
                .max()
                .unwrap();

            let g = game
                .draws
                .iter()
                .map(|draw| draw.get(&'g').unwrap_or(&0))
                .max()
                .unwrap();

            let b = game
                .draws
                .iter()
                .map(|draw| draw.get(&'b').unwrap_or(&0))
                .max()
                .unwrap();

            if r <= &MAX_R && g <= &MAX_G && b <= &MAX_B {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2() -> i32 {
    INPUT
        .lines()
        .filter_map(|line| {
            let game = parse_line(line);
            let r = game
                .draws
                .iter()
                .map(|draw| draw.get(&'r').unwrap_or(&0))
                .max()
                .unwrap();

            let g = game
                .draws
                .iter()
                .map(|draw| draw.get(&'g').unwrap_or(&0))
                .max()
                .unwrap();

            let b = game
                .draws
                .iter()
                .map(|draw| draw.get(&'b').unwrap_or(&0))
                .max()
                .unwrap();

            Some(r * g * b)
        })
        .sum()
}

struct Game {
    id: i32,
    draws: Vec<HashMap<char, i32>>,
}

fn parse_line(line: &str) -> Game {
    let mut line_splits = line.split(": ");
    let id = line_splits
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let draws = line_splits
        .next()
        .unwrap()
        .split("; ")
        .map(|draw_str| {
            draw_str
                .split(", ")
                .map(|draw| {
                    let mut draw_splits = draw.split(' ');
                    let count = draw_splits.next().unwrap().parse::<i32>().unwrap();
                    let color = draw_splits.next().unwrap().chars().next().unwrap();
                    assert!(draw_splits.next().is_none());
                    (color, count)
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    assert!(line_splits.next().is_none());

    Game { id, draws }
}
