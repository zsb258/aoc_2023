fn main() {
    let input: &str = include_str!("../../inputs/day7.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));

    let example: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    println!("{}", part1(example));
    println!("{}", part2(example));
}

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
    fn _new(cards: &[u8]) -> Self {
        use HandType as H;
        let mut counts = [0; 14 + 1];
        for &card in cards {
            counts[card as usize] += 1;
        }
        let mut counts = counts.iter().filter(|&&n| n > 0).collect::<Vec<_>>();
        counts.sort();

        dbg!(&counts);

        match counts.last().unwrap() {
            5 => H::FiveOfAKind,
            4 => H::FourOfAKind,
            3 => {
                if counts.len() == 2 {
                    H::FullHouse
                } else {
                    H::ThreeOfAKind
                }
            }
            2 => {
                if counts.len() == 3 {
                    H::TwoPair
                } else {
                    H::OnePair
                }
            }
            _ => H::HighCard,
        }
    }

    fn new(cards: &[u8]) -> Self {
        use HandType as H;
        let mut counts = [0; 14 + 1];
        let mut jokers = 0;
        for &card in cards {
            if card == 1 {
                jokers += 1;
            } else {
                counts[card as usize] += 1;
            }
        }

        let mut counts = counts.iter().filter(|&&n| n > 0).collect::<Vec<_>>();
        counts.sort();

        dbg!(&cards);
        dbg!(&counts, jokers);

        let highest = *counts.last().unwrap_or(&&0) + jokers;
        let unique_count = counts.len() + if jokers > 0 { 1 } else { 0 };
        dbg!(highest, unique_count);

        match highest {
            5 => H::FiveOfAKind,
            4 => H::FourOfAKind,
            3 => match jokers {
                0 => match counts[..] {
                    [2, 3] => H::FullHouse,
                    [1, 1, 3] => H::ThreeOfAKind,
                    _ => unreachable!(),
                },
                1 => match counts[..] {
                    [2, 2] => H::FullHouse,
                    [1, 1, 2] => H::ThreeOfAKind,
                    _ => unreachable!(),
                },
                2 => match counts[..] {
                    [1, 1, 1] => H::ThreeOfAKind,
                    _ => unreachable!(),
                },
                3 => match counts[..] {
                    [1, 1, 1] => H::ThreeOfAKind,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            2 => match jokers {
                0 => match counts[..] {
                    [1, 2, 2] => H::TwoPair,
                    [1, 1, 1, 2] => H::OnePair,
                    _ => unreachable!(),
                },
                1 => match counts[..] {
                    [1, 1, 1, 1] => H::OnePair,
                    _ => unreachable!(),
                },
                2 => match counts[..] {
                    [1, 1, 1] => H::OnePair,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            },
            _ => H::HighCard,
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut splits = line.splitn(2, ' ');

            let cards = splits
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>();
            let hand_type = HandType::new(&cards);

            Hand {
                cards,
                hand_type,
                bid: splits.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum()
}

fn part2(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut splits = line.splitn(2, ' ');

            let cards = splits
                .next()
                .unwrap()
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u8,
                })
                .collect::<Vec<_>>();
            let hand_type = HandType::new(&cards);

            Hand {
                cards,
                hand_type,
                bid: splits.next().unwrap().parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u64 + 1) * h.bid)
        .sum()
}

#[test]
fn example() {
    let example: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(example), 6440);
    assert_eq!(part2(example), 5905)
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day7.txt");
    assert_eq!(part1(input), 253933213);
    assert_eq!(part2(input), 253473930);
}

#[test]
fn test_handtype_new() {
    use HandType as H;
    fn helper(input: &str) -> Vec<u8> {
        input
            .chars()
            .map(|c| match c {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => c.to_digit(10).unwrap() as u8,
            })
            .collect::<Vec<_>>()
    }
    assert_eq!(H::new(&helper("3J4QA")), H::OnePair);
    assert_eq!(H::new(&helper("3J777")), H::FourOfAKind);
    assert_eq!(H::new(&helper("8J6TK")), H::OnePair);
    assert_eq!(H::new(&helper("99J9T")), H::FourOfAKind);
    assert_eq!(H::new(&helper("9JA4A")), H::ThreeOfAKind);
    assert_eq!(H::new(&helper("28J28")), H::FullHouse);
    assert_eq!(H::new(&helper("424KT")), H::OnePair);
}
