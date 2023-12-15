use std::collections::VecDeque;

fn main() {
    let input: &str = include_str!("../../inputs/day15.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .strip_suffix('\n')
        .unwrap_or(input)
        .split(',')
        .map(hash)
        .sum()
}

enum Op<'a> {
    Insert(&'a str, usize),
    Remove(&'a str),
}

fn part2(input: &str) -> usize {
    input
        .strip_suffix('\n')
        .unwrap_or(input)
        .split(',')
        .map(|word| {
            if word.contains('=') {
                let (label, num) = word.split_once('=').unwrap();
                let num = num.parse::<usize>().unwrap();
                Op::Insert(label, num)
            } else {
                let (label, _) = word.split_once('-').unwrap();
                Op::Remove(label)
            }
        })
        .fold(
            vec![VecDeque::new(); 256].as_mut(),
            |hm: &mut Vec<VecDeque<(&str, usize)>>, op| match op {
                Op::Insert(label, num) => {
                    let bucket = hash(label);
                    if let Some((_, prev)) = hm[bucket].iter_mut().find(|(k, _)| k == &label) {
                        *prev = num;
                    } else {
                        hm[bucket].push_back((label, num));
                    }
                    hm
                }
                Op::Remove(label) => {
                    hm[hash(label)].retain(|(k, _)| k != &label);
                    hm
                }
            },
        )
        .iter()
        .enumerate()
        .map(|(i, q)| {
            q.iter()
                .enumerate()
                .map(|(j, (_, val))| (i + 1) * (j + 1) * val)
                .sum::<usize>()
        })
        .sum()
}

fn hash(word: &str) -> usize {
    word.chars().fold(0, |mut hash, ch| {
        hash += ch as usize;
        hash *= 17;
        hash % 256
    })
}

#[test]
fn example() {
    let example: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(example), 1320);
    assert_eq!(part2(example), 145);
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
    assert_eq!(hash("rn"), 0);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day15.txt");
    assert_eq!(part1(input), 512283);
    assert_eq!(part2(input), 215827);
}
