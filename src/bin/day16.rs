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
    dir: (isize, isize),
}

fn one_beam(grid: &[Vec<char>], start: Beam) -> usize {
    let mut history = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    history.insert(start);
    let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

    while !queue.is_empty() {
        let beam = queue.pop_front().unwrap();
        let (mut r, mut c) = beam.pos;
        seen[r][c] = true;
        let (mut dr, mut dc) = beam.dir;

        loop {
            seen[r][c] = true;
            match grid[r][c] {
                '.' => {}
                '/' => {
                    let temp = dr;
                    dr = -dc;
                    dc = -temp;
                }
                '\\' => {
                    std::mem::swap(&mut dr, &mut dc);
                }
                '|' => {
                    if dr == 0 {
                        if !history.contains(&Beam {
                            pos: (r, c),
                            dir: (-1, 0),
                        }) {
                            queue.push_back(Beam {
                                pos: (r, c),
                                dir: (-1, 0),
                            });
                            history.insert(Beam {
                                pos: (r, c),
                                dir: (-1, 0),
                            });
                        }

                        if !history.contains(&Beam {
                            pos: (r, c),
                            dir: (1, 0),
                        }) {
                            queue.push_back(Beam {
                                pos: (r, c),
                                dir: (1, 0),
                            });
                            history.insert(Beam {
                                pos: (r, c),
                                dir: (1, 0),
                            });
                        }

                        break;
                    }
                }
                '-' => {
                    if dc == 0 {
                        if !history.contains(&Beam {
                            pos: (r, c),
                            dir: (0, -1),
                        }) {
                            queue.push_back(Beam {
                                pos: (r, c),
                                dir: (0, -1),
                            });
                            history.insert(Beam {
                                pos: (r, c),
                                dir: (0, -1),
                            });
                        }

                        if !history.contains(&Beam {
                            pos: (r, c),
                            dir: (0, 1),
                        }) {
                            queue.push_back(Beam {
                                pos: (r, c),
                                dir: (0, 1),
                            });
                            history.insert(Beam {
                                pos: (r, c),
                                dir: (0, 1),
                            });
                        }

                        break;
                    }
                }
                _ => unreachable!(),
            }
            if r as isize + dr < 0
                || r as isize + dr >= grid.len() as isize
                || c as isize + dc < 0
                || c as isize + dc >= grid[0].len() as isize
            {
                break;
            }
            r = (r as isize + dr) as usize;
            c = (c as isize + dc) as usize;
        }
    }

    // dbg!(&sen);
    // dbg!(&history);

    seen.iter().flatten().filter(|&&b| b).count()
}

fn part1(input: &str) -> usize {
    let grid = to_grid(input);
    one_beam(
        &grid,
        Beam {
            pos: (0, 0),
            dir: (0, 1),
        },
    )
}

fn part2(input: &str) -> usize {
    let grid = to_grid(input);
    let mut top = (0..grid[0].len()).map(|c| {
        one_beam(
            &grid,
            Beam {
                pos: (0, c),
                dir: (1, 0),
            },
        )
    });

    let mut bottom = (0..grid[0].len()).map(|c| {
        one_beam(
            &grid,
            Beam {
                pos: (grid.len() - 1, c),
                dir: (-1, 0),
            },
        )
    });
    let mut left = (0..grid.len()).map(|r| {
        one_beam(
            &grid,
            Beam {
                pos: (r, 0),
                dir: (0, 1),
            },
        )
    });
    let mut right = (0..grid.len()).map(|r| {
        one_beam(
            &grid,
            Beam {
                pos: (r, grid[0].len() - 1),
                dir: (0, -1),
            },
        )
    });

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
    // assert_eq!(part2(input), 215827);
}
