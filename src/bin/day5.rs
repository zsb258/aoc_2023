fn main() {
    let input: &str = include_str!("../../inputs/day5.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(PartialEq, Eq, Debug)]
struct Mapper {
    dst_start: u64,
    src_start: u64,
    length: u64,
}

impl PartialOrd for Mapper {
    fn partial_cmp(&self, other: &Mapper) -> Option<std::cmp::Ordering> {
        Some(self.src_start.cmp(&other.src_start))
    }
}

impl Ord for Mapper {
    fn cmp(&self, other: &Mapper) -> std::cmp::Ordering {
        self.src_start.cmp(&other.src_start)
    }
}

/// brute force
fn map_all(src: Vec<u64>, mappers: Vec<Mapper>) -> Vec<u64> {
    let mut dst = Vec::new();

    for &s in src.iter() {
        if let Some(m) = mappers
            .iter()
            .find(|m| s >= m.src_start && s < m.src_start + m.length)
        {
            dst.push(m.dst_start + s - m.src_start);
        } else {
            dst.push(s);
        }
    }

    dst
}

fn map_all_range(src: Vec<(u64, u64)>, mappers: Vec<Mapper>) -> Vec<(u64, u64)> {
    let mut src = src;
    src.sort();

    let mut mappers = mappers;
    mappers.sort();

    let mut dst = Vec::new();
    let mut j = 0;

    dbg!(&src);
    dbg!(&mappers);

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
                    if j < mappers.len() - 1 {
                        j += 1;
                    }
                }
            } else {
                if end >= m.src_start {
                    dst.push((curr, m.src_start - curr));
                    curr = m.src_start;
                } else {
                    dst.push((curr, end - curr));
                    break;
                }
            }
        }
    }

    dst
}

fn part1(input: &str) -> u64 {
    let mut splits1 = input.splitn(2, '\n');
    let seeds = splits1
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mappers_lst = splits1
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
        .collect::<Vec<Vec<Mapper>>>();

    let mut src = seeds;
    for mappers in mappers_lst {
        src = map_all(src, mappers);
    }

    *src.iter().min().unwrap()
}

fn part2(input: &str) -> u64 {
    let mut splits1 = input.splitn(2, '\n');
    let seeds = splits1
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>()[..]
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .collect::<Vec<_>>();

    let mappers_lst = splits1
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
        .collect::<Vec<Vec<Mapper>>>();

    dbg!(&seeds);
    dbg!(&mappers_lst);

    let mut src = seeds;
    for mappers in mappers_lst {
        src = map_all_range(src, mappers);
        dbg!(&src);
        // panic!("dbg");
    }

    src.iter().min().unwrap().0
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
    assert_eq!(part1(example), 35);
    assert_eq!(part2(example), 46);
}
