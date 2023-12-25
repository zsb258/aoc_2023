const INPUT: &str = include_str!("../../inputs/day01.txt");

fn main() {
    println!("Part1: {}", solve_with(&part1_parse_digits));
    println!("Part2: {}", solve_with(&part2_parse_alpha_digits));
}

fn solve_with(parser: &'_ dyn Fn(&str) -> Box<dyn Iterator<Item = u32> + '_>) -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let mut digits = parser(line);
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn part1_parse_digits(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    Box::new(line.chars().filter_map(|c| c.to_digit(10)))
}

fn part2_parse_alpha_digits(line: &str) -> Box<dyn Iterator<Item = u32> + '_> {
    Box::new((0..line.len()).filter_map(|i| {
        if let Some(n) = line[i..].chars().next().unwrap().to_digit(10) {
            Some(n)
        } else {
            let mut j = i + 2;

            while j < line.len() && j < i + 5 {
                if let Some(n) = match line[i..=j].to_string().as_str() {
                    "one" => Some(1),
                    "two" => Some(2),
                    "three" => Some(3),
                    "four" => Some(4),
                    "five" => Some(5),
                    "six" => Some(6),
                    "seven" => Some(7),
                    "eight" => Some(8),
                    "nine" => Some(9),
                    _ => None,
                } {
                    return Some(n);
                }
                j += 1;
            }
            None
        }
    }))
}
