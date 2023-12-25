fn main() {
    let input: &str = include_str!("../../inputs/day06.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn compute_one((t, d): (u64, u64)) -> u64 {
    let mut i = 1;
    while i * (t - i) <= d {
        i += 1;
    }
    t - i - i + 1
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
    });
    lines
        .next()
        .unwrap()
        .zip(lines.next().unwrap())
        .map(compute_one)
        .product()
}

fn part2(input: &str) -> u64 {
    compute_one(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .fold("".to_string(), |acc, s| acc + s)
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|v| (v[0], v[1]))
            .take(1)
            .next()
            .unwrap(),
    )
}

#[test]
fn example() {
    let example: &str = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(example), 288);
    assert_eq!(part2(example), 71503)
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day06.txt");
    assert_eq!(part1(input), 781200);
    assert_eq!(part2(input), 49240091);
}
