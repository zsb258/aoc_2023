use std::collections::HashMap;

use combination::v2::{Combine, Selector};

fn main() {
    let input: &str = include_str!("../../inputs/day12.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Clone, Debug)]
struct Row {
    records: Vec<Status>,
    rules: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Status {
    Unknown,
    Op,
    Damage,
}

fn part1(input: &str) -> u64 {
    let rows = input.lines().map(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let records = left
            .chars()
            .map(|c| match c {
                '?' => Status::Unknown,
                '.' => Status::Op,
                '#' => Status::Damage,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let rules = right
            .split(',')
            .map(|rule| rule.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Row { records, rules }
    });

    let mut memo: HashMap<(usize, usize, usize), u64> = HashMap::new();

    rows.map(|row| {
        memo.clear();
        dp(&row.records, None, &row.rules, &mut memo)
    })
    .sum()
}

fn part2(input: &str) -> u64 {
    let rows = input.lines().map(|line| {
        let (ori_left, ori_right) = line.split_once(' ').unwrap();
        let left = (0..5)
            .map(|_| ori_left.to_string())
            .collect::<Vec<String>>()
            .join("?");
        let right = (0..5)
            .map(|_| ori_right.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let records = left
            .chars()
            .map(|c| match c {
                '?' => Status::Unknown,
                '.' => Status::Op,
                '#' => Status::Damage,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let rules = right
            .split(',')
            .map(|rule| rule.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Row { records, rules }
    });

    let mut memo: HashMap<(usize, usize, usize), u64> = HashMap::new();

    rows.map(|row| {
        memo.clear();
        dp(&row.records, None, &row.rules, &mut memo)
    })
    .sum()
}

/// brute force
fn _compute_row(row: &Row) -> u64 {
    use Status as S;
    let remain: usize =
        row.rules.iter().sum::<usize>() - row.records.iter().filter(|x| **x == S::Damage).count();
    if remain == 0 {
        return 1;
    }

    let unknowns = row.records.iter().filter(|x| **x == S::Unknown).count();

    let mut sum = 0;

    let combi = Combine::new(unknowns, remain);

    for comb in combi.select_mode() {
        let mut records = row.records.clone();
        let mut comb_idx = 0;
        let mut unknown_count = 0;

        for i in 0..row.records.len() {
            if row.records[i] == S::Unknown {
                if comb[comb_idx] == unknown_count {
                    records[i] = S::Damage;
                    if comb_idx < comb.len() - 1 {
                        comb_idx += 1;
                    } else {
                        break;
                    }
                } else {
                    records[i] = S::Op;
                };
                unknown_count += 1;
            }
        }
        assert_eq!(comb_idx, comb.len() - 1);

        let mut flag = true;
        let mut partial_sum = 0;
        let mut j = 0;
        let mut check = vec![];
        while j < records.len() {
            if records[j] == S::Damage {
                partial_sum += 1;
            } else {
                if partial_sum != 0 {
                    if partial_sum != row.rules[check.len()] {
                        flag = false;
                        break;
                    }
                    check.push(partial_sum);
                }
                partial_sum = 0;
            }
            j += 1;
        }
        if partial_sum != 0 {
            if partial_sum != row.rules[check.len()] {
                flag = false;
            }
            check.push(partial_sum);
        }

        if flag {
            sum += 1;
        }
    }

    sum
}

fn dp(
    records: &[Status],
    consect_count: Option<usize>,
    rules: &[usize],
    memo: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    use Status as S;

    if records.is_empty() {
        dbg!(records, consect_count, rules);
        if let Some(x) = consect_count {
            if rules.len() == 1 && x == rules[0] {
                return 1;
            }
        } else if rules.is_empty() {
            return 1;
        }
        return 0;
    }

    if rules.is_empty() {
        return match consect_count {
            None => {
                if records.is_empty() || !records.iter().any(|x| *x == S::Damage) {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
    }

    let key = (records.len(), consect_count.unwrap_or(0), rules.len());
    if let Some(&x) = memo.get(&key) {
        return x;
    }

    let ret = match (&records[0], consect_count) {
        (S::Damage, Some(x)) => dp(&records[1..], Some(x + 1), rules, memo),
        (S::Damage, None) => dp(&records[1..], Some(1), rules, memo),
        (S::Op, Some(x)) => {
            if x != rules[0] {
                0
            } else {
                dp(&records[1..], None, &rules[1..], memo)
            }
        }
        (S::Op, None) => dp(&records[1..], None, rules, memo),
        (S::Unknown, Some(x)) => {
            dp(&records[1..], Some(x + 1), rules, memo)
                + if x != rules[0] {
                    0
                } else {
                    dp(&records[1..], None, &rules[1..], memo)
                }
        }
        (S::Unknown, None) => {
            dp(&records[1..], Some(1), rules, memo) + dp(&records[1..], None, rules, memo)
        }
    };

    memo.insert(key, ret);

    dbg!(records, consect_count, rules, ret);
    ret
}

#[test]
fn example() {
    let example: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    assert_eq!(part1(example), 21);
    assert_eq!(part2(example), 525152);
}

#[test]
fn test_rows() {
    assert_eq!(part1("????? 1,1"), 6);
    assert_eq!(part1("?????.?##?? 1,1,4"), 12);
    assert_eq!(part1("??..#??#.??? 1,1"), 1);

    assert_eq!(part2("???.### 1,1,3"), 1);
    assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
    assert_eq!(part2("?###???????? 3,2,1"), 506250);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day12.txt");
    assert_eq!(part1(input), 6827);
    assert_eq!(part2(input), 1537505634471);
}
