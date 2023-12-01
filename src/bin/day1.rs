const INPUT: &str = include_str!("../../inputs/day1.txt");

fn main() {
    println!("Part1: {}", part1());
    println!("Part2: {}", part2());
}

fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn part2() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let digits = parse_alpha_digits(line);
            let first = digits.first().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn parse_alpha_digits(line: &str) -> Vec<u32> {
    let mut i = 0;
    let mut digits = Vec::new();
    while i < line.len() {
        if let Some(d) = line[i..].chars().next().unwrap().to_digit(10) {
            digits.push(d);
            i += 1;
            continue;
        }

        let mut j = i + 2;

        while j < line.len() && j < i + 5 {
            match line[i..=j].to_string().as_str() {
                "one" => digits.push(1),
                "two" => digits.push(2),
                "three" => digits.push(3),
                "four" => digits.push(4),
                "five" => digits.push(5),
                "six" => digits.push(6),
                "seven" => digits.push(7),
                "eight" => digits.push(8),
                "nine" => digits.push(9),
                _ => (),
            }
            j += 1;
        }

        i += 1;
    }

    digits
}
