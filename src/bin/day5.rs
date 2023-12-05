fn main() {
    let input: &str = include_str!("../../inputs/day5.txt");
    println!("Part1: {}", solve_with(input, &part1_seeds_parser));
    println!("Part2: {}", solve_with(input, &part2_seeds_parser));
}

#[derive(Debug)]
struct Mapper {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

/// src and mappers vectors are sorted by start position
fn map_all_range(src: Vec<(u64, u64)>, mappers: Vec<Mapper>) -> Vec<(u64, u64)> {
    let mut dst = Vec::new();
    let mut j = 0;

    for s in src {
        let end = s.0 + s.1;
        let mut curr = s.0;

        while curr < end {
            let m = &mappers[j];
            if curr >= m.src_start + m.length {
                if j < mappers.len() - 1 {
                    j += 1;
                } else {
                    dst.push((curr, end - curr));
                    break;
                }
            } else if curr >= m.src_start {
                if end <= m.src_start + m.length {
                    dst.push((m.dst_start + curr - m.src_start, end - curr));
                    curr = end;
                } else {
                    let delta = m.src_start + m.length - curr;
                    dst.push((m.dst_start + curr - m.src_start, delta));
                    curr += delta;
                }
            } else if end >= m.src_start {
                dst.push((curr, m.src_start - curr));
                curr = m.src_start;
            } else {
                dst.push((curr, end - curr));
                break;
            }
        }
    }

    dst
}

fn part1_seeds_parser(input: &str) -> Vec<(u64, u64)> {
    input
        .split_whitespace()
        .map(|x| (x.parse::<u64>().unwrap(), 1_u64))
        .collect::<Vec<_>>()
}

fn part2_seeds_parser(input: &str) -> Vec<(u64, u64)> {
    input
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()[..]
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>()
}

fn solve_with(input: &str, seeds_parser: &'_ dyn Fn(&'_ str) -> Vec<(u64, u64)>) -> u64 {
    let mut splits = input.splitn(2, '\n');
    let seeds = seeds_parser(splits.next().unwrap().split(": ").nth(1).unwrap());

    splits
        .next()
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.split(":\n")
                .nth(1)
                .unwrap()
                .lines()
                .map(|line| {
                    if let [dst_start, src_start, length] = line
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<_>>()[..]
                    {
                        Mapper {
                            dst_start,
                            src_start,
                            length,
                        }
                    } else {
                        panic!("invalid input")
                    }
                })
                .collect::<Vec<Mapper>>()
        })
        .fold(seeds, |mut src, mut mappers| {
            src.sort();
            mappers.sort_by(|a, b| a.src_start.cmp(&b.src_start));
            map_all_range(src, mappers)
        })
        .iter()
        .min()
        .unwrap()
        .0
}

#[test]
fn example() {
    let example: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(solve_with(example, &part1_seeds_parser), 35);
    assert_eq!(solve_with(example, &part2_seeds_parser), 46);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day5.txt");
    assert_eq!(solve_with(input, &part1_seeds_parser), 84470622);
    assert_eq!(solve_with(input, &part2_seeds_parser), 26714516);
}
