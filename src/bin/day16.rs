use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../../inputs/day16.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Beam {
    pos: (usize, usize),
    dir: Dir,
}

impl Beam {
    fn new(pos: (usize, usize), dir: Dir) -> Self {
        Self { pos, dir }
    }

    fn step_checked(&self, max_r: usize, max_c: usize) -> Option<Self> {
        let (r, c) = self.pos;
        let (dr, dc) = self.dir.as_tuple();
        if r as isize + dr < 0
            || r as isize + dr >= max_r as isize
            || c as isize + dc < 0
            || c as isize + dc >= max_c as isize
        {
            None
        } else {
            Some(Self {
                pos: (
                    (self.pos.0 as isize + dr) as usize,
                    (self.pos.1 as isize + dc) as usize,
                ),
                dir: self.dir,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    /// forward slash
    fn reflect_forward(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    /// back slash
    fn reflect_backward(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }

    fn as_tuple(&self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }
}

fn one_beam(grid: &[Vec<char>], start: Beam) -> usize {
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

    let mut history = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    history.insert(start);

    while !queue.is_empty() {
        let mut beam = queue.pop_front().unwrap();

        loop {
            seen[beam.pos.0][beam.pos.1] = true;

            match grid[beam.pos.0][beam.pos.1] {
                '.' => (),
                '/' => {
                    beam.dir = beam.dir.reflect_forward();
                }
                '\\' => {
                    beam.dir = beam.dir.reflect_backward();
                }
                '|' => {
                    if beam.dir == Dir::Left || beam.dir == Dir::Right {
                        let up = Beam::new(beam.pos, Dir::Up);
                        if !history.contains(&up) {
                            queue.push_back(up);
                            history.insert(up);
                        }

                        let down = Beam::new(beam.pos, Dir::Down);
                        if !history.contains(&down) {
                            queue.push_back(down);
                            history.insert(down);
                        }

                        break;
                    }
                }
                '-' => {
                    if beam.dir == Dir::Up || beam.dir == Dir::Down {
                        let left = Beam::new(beam.pos, Dir::Left);
                        if !history.contains(&left) {
                            queue.push_back(left);
                            history.insert(left);
                        }

                        let right = Beam::new(beam.pos, Dir::Right);
                        if !history.contains(&right) {
                            queue.push_back(right);
                            history.insert(right);
                        }

                        break;
                    }
                }
                _ => unreachable!(),
            }

            if let Some(new_beam) = beam.step_checked(grid.len(), grid[0].len()) {
                beam = new_beam;
            } else {
                break;
            }
        }
    }

    seen.iter().flatten().filter(|&&b| b).count()
}

fn part1(input: &str) -> usize {
    let grid = to_grid(input);
    one_beam(&grid, Beam::new((0, 0), Dir::Right))
}

fn part2(input: &str) -> usize {
    let grid = to_grid(input);

    let top = (0..grid[0].len()).map(|c| one_beam(&grid, Beam::new((0, c), Dir::Down)));

    let bottom =
        (0..grid[0].len()).map(|c| one_beam(&grid, Beam::new((grid.len() - 1, c), Dir::Up)));

    let left = (0..grid.len()).map(|r| one_beam(&grid, Beam::new((r, 0), Dir::Right)));

    let right =
        (0..grid.len()).map(|r| one_beam(&grid, Beam::new((r, grid[0].len() - 1), Dir::Left)));

    top.chain(bottom).chain(left).chain(right).max().unwrap()
}

#[test]
fn example() {
    let example: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    assert_eq!(part1(example), 46);
    assert_eq!(part2(example), 51);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day16.txt");
    assert_eq!(part1(input), 8551);
    assert_eq!(part2(input), 8754);
}
