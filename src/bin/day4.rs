fn main() {
    let input: &str = include_str!("../../inputs/day4.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut it = line
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
                        .collect::<Vec<u32>>()
                });
            let win_nums = it.next().unwrap();
            // println!("{:?}", win_nums);
            let nums = it.next().unwrap();

            let mut common_count: u32 = 0;
            for num in nums {
                if win_nums.contains(&num) {
                    common_count += 1;
                }
            }
            if common_count > 0 {
                2_u32.pow(common_count - 1)
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let counts = input
        .lines()
        .map(|line| {
            let mut it = line
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
                        .collect::<Vec<u32>>()
                });
            let win_nums = it.next().unwrap();
            // println!("{:?}", win_nums);
            let nums = it.next().unwrap();

            let mut common_count: u32 = 0;
            for num in nums {
                if win_nums.contains(&num) {
                    common_count += 1;
                }
            }
            common_count
        })
        .collect::<Vec<u32>>();

    let mut copies = vec![1_u32; counts.len()];
    for (i, count) in counts.iter().enumerate() {
        for j in 1..=*count {
            copies[i + j as usize] += copies[i];
        }
    }

    copies.iter().sum()
}

#[test]
fn example() {
    let example: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(example), 13);
    assert_eq!(part2(example), 30);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day4.txt");
    assert_eq!(part1(input), 26914);
    assert_eq!(part2(input), 13080971);
}
