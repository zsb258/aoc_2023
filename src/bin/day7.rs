use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    hand_type: HandType,
    bid: u64,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        use std::cmp::Ordering as O;
        match self.hand_type.cmp(&other.hand_type) {
            O::Less => O::Less,
            O::Greater => O::Greater,
            O::Equal => self.cards.cmp(&other.cards),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    fn new(cards: &[u8]) -> Self {
        let jokers = cards.iter().filter(|&&n| n == 1).count();

        let counts = cards
            .iter()
            .filter(|&&n| n != 1)
            .fold(HashMap::new(), |mut acc, &card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            })
            .iter()
            .map(|(_, &n)| n)
            .sorted()
            .collect::<Vec<_>>();

        match (counts.last().unwrap_or(&0) + jokers, counts.len()) {
            (5, _) => Self::FiveOfAKind,
            (4, _) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, 3) => Self::ThreeOfAKind,
            (2, 3) => Self::TwoPair,
            (2, 4) => Self::OnePair,
            (1, 5) => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input: &str = include_str!("../../inputs/day7.txt");
    println!("Part1: {}", solve_with(input, &part1_char_mapper));
    println!("Part2: {}", solve_with(input, &part2_char_mapper));
}

fn part1_char_mapper(c: char) -> u8 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn part2_char_mapper(c: char) -> u8 {
    match c {
        'T' => 10,
        'J' => 1, // Joker
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn solve_with<'a>(input: &'a str, char_mapper: &dyn Fn(char) -> u8) -> u64 {
    input
        .lines()
        .map(|line: &'a str| {
            let mut splits = line.splitn(2, ' ');

            let cards = splits
                .next()
                .unwrap()
                .chars()
                .map(char_mapper)
                .collect::<Vec<_>>();
            let hand_type = HandType::new(&cards);

            Hand {
                cards,
                hand_type,
                bid: splits.next().unwrap().parse().unwrap(),
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum()
}

#[test]
fn test_part2_parse_handtype() {
    use HandType as H;

    fn helper(input: &str) -> H {
        H::new(
            &input
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>(),
        )
    }

    assert_eq!(helper("3J4QA"), H::OnePair);
    assert_eq!(helper("3J777"), H::FourOfAKind);
    assert_eq!(helper("8J6TK"), H::OnePair);
    assert_eq!(helper("99J9T"), H::FourOfAKind);
    assert_eq!(helper("9JA4A"), H::ThreeOfAKind);
    assert_eq!(helper("28J28"), H::FullHouse);
    assert_eq!(helper("424KT"), H::OnePair);
}

#[test]
fn example() {
    let example: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(solve_with(example, &part1_char_mapper), 6440);
    assert_eq!(solve_with(example, &part2_char_mapper), 5905);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day7.txt");
    assert_eq!(solve_with(input, &part1_char_mapper), 253933213);
    assert_eq!(solve_with(input, &part2_char_mapper), 253473930);
}
