use std::collections::HashMap;

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

fn parse<'a>(input: &'a str, duplicates: &'a u32) -> impl Iterator<Item = Row> + 'a {
    input.lines().map(|line| {
        let (left, ori_right) = line.split_once(' ').unwrap();

        let records = (0..*duplicates)
            .map(|_| left.to_string())
            .collect::<Vec<String>>()
            .join("?")
            .chars()
            .map(|c| match c {
                '?' => Status::Unknown,
                '.' => Status::Op,
                '#' => Status::Damage,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        let rules = (0..*duplicates)
            .map(|_| ori_right.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .split(',')
            .map(|rule| rule.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Row { records, rules }
    })
}

fn part1(input: &str) -> u64 {
    parse(input, &1)
        .map(|row| {
            let mut memo: HashMap<(usize, usize, usize), u64> = HashMap::new();
            dp(&row.records, None, &row.rules, &mut memo)
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    parse(input, &5)
        .map(|row| {
            let mut memo: HashMap<(usize, usize, usize), u64> = HashMap::new();
            dp(&row.records, None, &row.rules, &mut memo)
        })
        .sum()
}

fn dp(
    records: &[Status],
    consect_count: Option<usize>,
    rules: &[usize],
    memo: &mut HashMap<(usize, usize, usize), u64>,
) -> u64 {
    use Status as S;

    if records.is_empty() {
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
