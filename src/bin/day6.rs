fn main() {
    let input: &str = include_str!("../../inputs/day6.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse::<u32>().unwrap())
    });
    // let mut time = lines.next().unwrap().split_whitespace().skip(1);
    // let mut distance = lines.next().unwrap().split_whitespace().skip(1);
    let mut paired = lines.next().unwrap().zip(lines.next().unwrap());
    paired
        .map(|(t, d)| {
            let mut count = 0;
            for i in 1..t {
                if i * (t - i) > d {
                    count += 1;
                }
            }
            count
        })
        .fold(1, |acc, n| acc * n)
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .fold("".to_string(), |acc, s| acc + s)
            .parse::<u64>()
            .unwrap()
    });
    // let mut time = lines.next().unwrap().split_whitespace().skip(1);
    // let mut distance = lines.next().unwrap().split_whitespace().skip(1);
    let (t, d) = (lines.next().unwrap(), lines.next().unwrap());
    let mut count = 0;
    for i in 1..t {
        if i * (t - i) > d {
            count += 1;
        }
    }
    count
}

#[test]
fn example() {
    let example: &str = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(example), 288);
    assert_eq!(part2(example), 71503)
}
