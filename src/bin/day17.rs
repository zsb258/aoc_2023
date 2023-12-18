use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input: &str = include_str!("../../inputs/day17.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Nil,
}

impl Dir {
    fn directions() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    fn is_opposite(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Up, Self::Down)
                | (Self::Down, Self::Up)
                | (Self::Left, Self::Right)
                | (Self::Right, Self::Left)
        )
    }

    fn step(&self, (r, c): (usize, usize), dist: isize) -> (usize, usize) {
        let (r, c) = (r as isize, c as isize);
        let ret = match self {
            Self::Up => (r - dist, c),
            Self::Down => (r + dist, c),
            Self::Left => (r, c - dist),
            Self::Right => (r, c + dist),
            Self::Nil => (r, c),
        };
        (ret.0 as usize, ret.1 as usize)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    coord: (usize, usize),
    dir: Dir,
    val: isize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    dijkstra(&parse(input), 1, 3) as usize
}

fn part2(input: &str) -> usize {
    dijkstra(&parse(input), 4, 10) as usize
}

fn dijkstra(grid: &[Vec<usize>], minstep: isize, maxstep: isize) -> isize {
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        coord: (0, 0),
        dir: Dir::Nil,
        val: 0,
    });

    let mut memo = HashMap::new();

    while let Some(Node { coord, dir, val }) = heap.pop() {
        if coord == (grid.len() - 1, grid[0].len() - 1) {
            return val;
        }

        if memo.get(&(coord, dir)).is_some_and(|&v| val > v) {
            continue;
        }

        let (r, c) = coord;

        for next_dir in Dir::directions() {
            if dir == next_dir || dir.is_opposite(&next_dir) {
                continue;
            }

            let mut next_val = val;
            for dist in 1..=maxstep {
                let (rr, cc) = next_dir.step((r, c), dist);
                if rr >= grid.len() || cc >= grid[0].len() {
                    continue;
                }
                next_val += grid[rr][cc] as isize;
                if dist < minstep {
                    continue;
                }
                let coord = (rr, cc);
                if next_val < *memo.get(&(coord, next_dir)).unwrap_or(&isize::MAX) {
                    memo.insert((coord, next_dir), next_val);
                    heap.push(Node {
                        coord,
                        dir: next_dir,
                        val: next_val,
                    });
                }
            }
        }
    }

    unreachable!()
}

#[test]
fn example() {
    let example: &str = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(part1(example), 102);
    assert_eq!(part2(example), 94);
}

#[test]
fn part1_test_case() {
    let example: &str = r"241343231
321545353";
    assert_eq!(part1(example), 32);
}

#[test]
fn part2_test_cases() {
    let example: &str = r"11111111111";
    assert_eq!(part2(example), 10);

    let example: &str = r"11111111
99999999
99999999
99999999
99999999";
    assert_eq!(part2(example), 43);

    let example: &str = r"111111111111
999999999991
999999999991
999999999991
999999999991";
    assert_eq!(part2(example), 71);

    let example: &str = r"9111199911111
9999199919991
9999199919991
9999199919991
9999111119991";

    assert_eq!(part2(example), 24);
}

#[test]
#[should_panic]
fn part2_test_cases_negative() {
    let example: &str = r"111111111111";
    part2(example);

    let example: &str = r"1111111111111";
    part2(example);

    let example: &str = r"1111111111111111";
    part2(example);

    let example: &str = r"1
1
1
1
1
1
1
1
1
1
1
1";
    part2(example);

    let example: &str = r"1
1
1
1
1
1
1
1
1
1
1
1
1";
    part2(example);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day17.txt");
    assert_eq!(part1(input), 1099);
    assert_eq!(part2(input), 1266);
}
