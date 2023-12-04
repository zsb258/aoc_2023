fn main() {
    let input: &str = include_str!("../../inputs/day4.txt");
    println!("Part1: {}", solve_with(input, &part1_agg));
    println!("Part2: {}", solve_with(input, &part2_agg));
}

fn part1_agg(it: Box<dyn Iterator<Item = u32> + '_>) -> u32 {
    it.filter(|n| *n > 0).map(|num| 2_u32.pow(num - 1)).sum()
}

fn part2_agg(it: Box<dyn Iterator<Item = u32> + '_>) -> u32 {
    let counts = it.collect::<Vec<u32>>();
    let mut copies = vec![1_u32; counts.len()];
    for (i, count) in counts.iter().enumerate() {
        for j in 1..=*count {
            copies[i + j as usize] += copies[i];
        }
    }
    copies.iter().sum()
}

fn solve_with(input: &str, agg: &'_ dyn Fn(Box<dyn Iterator<Item = u32> + '_>) -> u32) -> u32 {
    agg(Box::new(input.lines().map(|line| {
        match &line
            .splitn(2, ':')
            .last()
            .unwrap()
            .splitn(2, '|')
            .map(|nums_str| {
                nums_str
                    .split_whitespace()
                    .filter_map(|num| match num.parse::<u32>() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()[..]
        {
            [win_nums, my_nums] => {
                my_nums.iter().filter(|num| win_nums.contains(num)).count() as u32
            }
            _ => unreachable!(),
        }
    })))
}

#[test]
fn example() {
    let example: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(solve_with(example, &part1_agg), 13);
    assert_eq!(solve_with(example, &part2_agg), 30);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day4.txt");
    assert_eq!(solve_with(input, &part1_agg), 26914);
    assert_eq!(solve_with(input, &part2_agg), 13080971);
}
