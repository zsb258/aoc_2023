use std::collections::VecDeque;

const START: (usize, usize) = (90, 62); // from input

fn main() {
    let input: &str = include_str!("../../inputs/day10.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input, START));
}

fn part1(input: &str) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = if grid.len() == 140 { START } else { (2, 0) }; // from example and actual inputs

    bfs(&grid, start).0 as u64
}

fn part2(input: &str, start: (usize, usize)) -> u64 {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (_, steps, end) = bfs(&grid, start);
    let mut is_route = vec![vec![false; grid[0].len()]; grid.len()];
    is_route[start.0][start.1] = true;
    is_route[end.0][end.1] = true;

    dbg!(&steps);
    dbg!(&end);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back(end);

    while let Some((r, c)) = queue.pop_front() {
        is_route[r][c] = true;
        let ch = grid[r][c];
        let next_step = steps[r][c] - 1;
        if is_facing('N', ch)
            && r > 0
            && is_facing('S', grid[r - 1][c])
            && steps[r - 1][c] == next_step
        {
            queue.push_back((r - 1, c));
        }

        if is_facing('S', ch)
            && r + 1 < grid.len()
            && is_facing('N', grid[r + 1][c])
            && steps[r + 1][c] == next_step
        {
            queue.push_back((r + 1, c));
        }

        if is_facing('E', ch)
            && c + 1 < grid[0].len()
            && is_facing('W', grid[r][c + 1])
            && steps[r][c + 1] == next_step
        {
            queue.push_back((r, c + 1));
        }

        if is_facing('W', ch)
            && c > 0
            && is_facing('E', grid[r][c - 1])
            && steps[r][c - 1] == next_step
        {
            queue.push_back((r, c - 1));
        }
    }

    dbg!(&is_route);
    let mut grid = grid.clone();
    let s_type = {
        let north = if start.0 > 0 && is_facing('S', grid[start.0 - 1][start.1]) {
            true
        } else {
            false
        };
        let south = if start.0 + 1 < grid.len() && is_facing('N', grid[start.0 + 1][start.1]) {
            true
        } else {
            false
        };
        let east = if start.1 + 1 < grid[0].len() && is_facing('W', grid[start.0][start.1 + 1]) {
            true
        } else {
            false
        };
        let west = start.1 > 0 && is_facing('E', grid[start.0][start.1 - 1]);

        dbg!("start", (north, south, east, west));

        match (north, south, east, west) {
            (true, false, true, false) => 'L',
            (true, false, false, true) => 'J',
            (false, true, true, false) => 'F',
            (false, true, false, true) => '7',
            _ => unreachable!(),
        }
    };
    grid[start.0][start.1] = s_type;

    let mut count = 0;
    for r in 0..is_route.len() {
        for c in 0..is_route[0].len() {
            if !is_route[r][c] {
                let dbg_flag = (r, c) == (4, 7);

                let mut i = r;
                let mut j = c;

                let mut edge_count = 0;
                let mut prev_corner = None;

                while i > 0 {
                    i -= 1;
                    if is_route[i][j] {
                        match grid[i][j] {
                            'S' => {
                                if prev_corner.is_none() {
                                    prev_corner = Some('S');
                                } else {
                                    prev_corner.take();
                                    edge_count += 1;
                                }
                            }
                            ch if ch == 'L' || ch == 'J' => {
                                prev_corner = Some(ch);
                            }
                            '7' => match prev_corner.take() {
                                Some('L') | Some('S') => edge_count += 1,
                                _ => (),
                            },

                            'F' => match prev_corner.take() {
                                Some('J') | Some('S') => {
                                    edge_count += 1;
                                }
                                _ => (),
                            },

                            '-' => edge_count += 1,
                            _ => {}
                        }
                    }
                }
                let north = edge_count > 0 && edge_count % 2 == 1;
                if dbg_flag {
                    dbg!(north, edge_count, prev_corner);
                }

                i = r;
                edge_count = 0;
                prev_corner = None;
                while i + 1 < is_route.len() {
                    i += 1;
                    if is_route[i][j] {
                        match grid[i][j] {
                            'S' => {
                                if prev_corner.is_none() {
                                    prev_corner = Some('S');
                                } else {
                                    prev_corner.take();
                                    edge_count += 1;
                                }
                            }
                            ch if ch == '7' || ch == 'F' => {
                                prev_corner = Some(ch);
                            }
                            'L' => match prev_corner.take() {
                                Some('7') | Some('S') => edge_count += 1,
                                _ => (),
                            },

                            'J' => match prev_corner.take() {
                                Some('F') | Some('S') => {
                                    edge_count += 1;
                                }
                                _ => (),
                            },

                            '-' => edge_count += 1,
                            _ => {}
                        }
                    }
                }
                let south = edge_count > 0 && edge_count % 2 == 1;
                if dbg_flag {
                    dbg!(south, edge_count, prev_corner);
                }

                i = r;
                edge_count = 0;
                prev_corner = None;
                while j > 0 {
                    j -= 1;
                    if is_route[i][j] {
                        match grid[i][j] {
                            'S' => {
                                if prev_corner.is_none() {
                                    prev_corner = Some('S');
                                } else {
                                    prev_corner.take();
                                    edge_count += 1;
                                }
                            }
                            ch if ch == '7' || ch == 'J' => {
                                prev_corner = Some(ch);
                            }
                            'L' => match prev_corner.take() {
                                Some('7') | Some('S') => edge_count += 1,
                                _ => (),
                            },

                            'F' => match prev_corner.take() {
                                Some('J') | Some('S') => {
                                    edge_count += 1;
                                }
                                _ => (),
                            },

                            '|' => edge_count += 1,
                            _ => {}
                        }
                    }
                }
                let west = edge_count > 0 && edge_count % 2 == 1;
                if dbg_flag {
                    dbg!(west, edge_count, prev_corner);
                }

                j = c;
                edge_count = 0;
                prev_corner = None;
                while j + 1 < is_route[0].len() {
                    j += 1;
                    if is_route[i][j] {
                        match grid[i][j] {
                            'S' => {
                                if prev_corner.is_none() {
                                    prev_corner = Some('S');
                                } else {
                                    prev_corner.take();
                                    edge_count += 1;
                                }
                            }
                            ch if ch == 'L' || ch == 'F' => {
                                prev_corner = Some(ch);
                            }
                            '7' => match prev_corner.take() {
                                Some('L') | Some('S') => edge_count += 1,
                                _ => (),
                            },

                            'J' => match prev_corner.take() {
                                Some('F') | Some('S') => {
                                    edge_count += 1;
                                }
                                _ => (),
                            },

                            '|' => edge_count += 1,
                            _ => {}
                        }
                    }
                }
                let east = edge_count > 0 && edge_count % 2 == 1;
                if dbg_flag {
                    dbg!(east, edge_count, prev_corner);
                }

                if dbg_flag {
                    dbg!((r, c, north, south, east, west));
                }
                if north && south && east && west {
                    dbg!("Inside", (r, c));
                }
                count += if north && south && east && west { 1 } else { 0 };
            }
        }
    }

    count
}

fn is_facing(dir: char, ch: char) -> bool {
    if ch == 'S' {
        return true;
    }
    match dir {
        'N' => ch == '|' || ch == 'L' || ch == 'J',
        'S' => ch == '|' || ch == '7' || ch == 'F',
        'E' => ch == '-' || ch == 'L' || ch == 'F',
        'W' => ch == '-' || ch == 'J' || ch == '7',
        _ => unreachable!(),
    }
}

fn bfs(grid: &[Vec<char>], start: (usize, usize)) -> (isize, Vec<Vec<isize>>, (usize, usize)) {
    let mut steps: Vec<Vec<isize>> = vec![vec![-1; grid[0].len()]; grid.len()];
    steps[start.0][start.1] = 0;
    let mut queue: VecDeque<((usize, usize), (usize, usize))> = VecDeque::new();
    queue.push_back((start, start));
    while let Some(((r, c), prev)) = queue.pop_front() {
        let ch = grid[r][c];

        if is_facing('N', ch) && r > 0 && prev != (r - 1, c) && is_facing('S', grid[r - 1][c]) {
            if steps[r - 1][c] > 0 {
                return (steps[r - 1][c], steps, (r - 1, c));
            }
            steps[r - 1][c] = steps[r][c] + 1;
            queue.push_back(((r - 1, c), (r, c)));
        }
        if is_facing('S', ch)
            && r + 1 < grid.len()
            && prev != (r + 1, c)
            && is_facing('N', grid[r + 1][c])
        {
            if steps[r + 1][c] > 0 {
                return (steps[r + 1][c], steps, (r + 1, c));
            }
            steps[r + 1][c] = steps[r][c] + 1;
            queue.push_back(((r + 1, c), (r, c)));
        }
        if is_facing('E', ch)
            && c + 1 < grid[0].len()
            && prev != (r, c + 1)
            && is_facing('W', grid[r][c + 1])
        {
            if steps[r][c + 1] > 0 {
                return (steps[r][c + 1], steps, (r, c + 1));
            }
            steps[r][c + 1] = steps[r][c] + 1;
            queue.push_back(((r, c + 1), (r, c)));
        }
        if is_facing('W', ch) && c > 0 && prev != (r, c - 1) && is_facing('E', grid[r][c - 1]) {
            if steps[r][c - 1] > 0 {
                return (steps[r][c - 1], steps, (r, c - 1));
            }
            steps[r][c - 1] = steps[r][c] + 1;
            queue.push_back(((r, c - 1), (r, c)));
        }
    }
    unreachable!()
}

#[test]
fn part1_example() {
    let example: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part1(example), 8);
}

#[test]
fn part2_example0() {
    let example: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part2(example, (2, 0)), 1);
}

#[test]
fn part2_example1() {
    let example: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(part2(example, (1, 1)), 4);
}

#[test]
fn part2_example2() {
    let example: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert_eq!(part2(example, (4, 12)), 8);
}

#[test]
fn part2_example3() {
    let example: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_eq!(part2(example, (0, 4)), 10);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day10.txt");
    assert_eq!(part1(input), 6800);
    // assert_eq!(part2(input), 49240091);
}
