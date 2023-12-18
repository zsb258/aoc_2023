fn main() {
    let input: &str = include_str!("../../inputs/day18.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

struct Instruction {
    dir: char,
    distance: usize,
    color: String,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut splits = line.split_whitespace();
            let dir = splits.next().unwrap().chars().next().unwrap();
            let distance = splits.next().unwrap().parse::<usize>().unwrap();
            let color = splits
                .next()
                .unwrap()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap();
            Instruction {
                dir,
                distance,
                color: color.to_string(),
            }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let instructions = parse(input);

    let horizontal: usize = instructions
        .iter()
        .filter(|i| i.dir == 'L' || i.dir == 'R')
        .map(|i| i.distance)
        .sum();
    let vertical: usize = instructions
        .iter()
        .filter(|i| i.dir == 'U' || i.dir == 'D')
        .map(|i| i.distance)
        .sum();

    // dbg!(horizontal, vertical);

    let mut grid = vec![vec!['.'; horizontal * 2]; vertical * 2];
    let (r_s, c_s) = (horizontal, vertical);

    grid[r_s][c_s] = '#';
    let mut r = r_s;
    let mut c = c_s;

    for instr in instructions {
        match instr.dir {
            'U' => {
                for _ in 0..instr.distance {
                    r -= 1;
                    grid[r][c] = '#';
                }
            }
            'D' => {
                for _ in 0..instr.distance {
                    r += 1;
                    grid[r][c] = '#';
                }
            }
            'L' => {
                for _ in 0..instr.distance {
                    c -= 1;
                    grid[r][c] = '#';
                }
            }
            'R' => {
                for _ in 0..instr.distance {
                    c += 1;
                    grid[r][c] = '#';
                }
            }
            _ => unreachable!(),
        }
    }

    let mut count = 0;
    let mut check = grid.clone();

    let r_min = grid
        .iter()
        .position(|row| row.iter().any(|ch| *ch == '#'))
        .unwrap();
    let r_max = grid
        .iter()
        .rposition(|row| row.iter().any(|ch| *ch == '#'))
        .unwrap();
    let c_min = grid[0]
        .iter()
        .enumerate()
        .position(|(c, _)| grid.iter().any(|row| row[c] == '#'))
        .unwrap();
    let c_max = grid[0]
        .iter()
        .enumerate()
        .rposition(|(c, _)| grid.iter().any(|row| row[c] == '#'))
        .unwrap();

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '#' {
                count += 1;
                // check[r][c] = 'O';
            } else if r >= r_min && r <= r_max && c >= c_min && c <= c_max {
                let debug = (r, c) == (r_min + 282 - 1, c_min + 89 - 1);
                if is_enclosed(&grid, (r, c), debug) {
                    count += 1;
                    check[r][c] = 'O';
                }
            }
        }
        if r % 100 == 0 {
            dbg!(r);
        }
    }

    use std::fs::File;
    use std::io::Write;
    let mut file = File::create("day18_ans.txt").unwrap();
    for (r, row) in check.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if r >= r_min && r <= r_max && c >= c_min && c <= c_max {
                write!(file, "{}", ch).unwrap();
            }
        }
        if r >= r_min && r <= r_max {
            writeln!(file).unwrap();
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let instructions = parse(input)
        .iter()
        .map(|instr| {
            let hex = instr.color.clone();
            // skip #
            let first_five = &hex[1..6];
            let last_char = &hex[6..].chars().next().unwrap();
            // dbg!(first_five, last_char);

            let dist = usize::from_str_radix(first_five, 16).unwrap();
            let dir = match last_char {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };

            Instruction {
                dir,
                distance: dist,
                color: hex,
            }
        })
        .collect::<Vec<_>>();

    // let horizontal: usize = instructions
    //     .iter()
    //     .filter(|i| i.dir == 'L' || i.dir == 'R')
    //     .map(|i| i.distance)
    //     .sum();
    // let vertical: usize = instructions
    //     .iter()
    //     .filter(|i| i.dir == 'U' || i.dir == 'D')
    //     .map(|i| i.distance)
    //     .sum();
    //
    // dbg!(horizontal, vertical);

    // let mut grid = vec![vec!['.'; horizontal * 2]; vertical * 2];
    let (r_s, c_s) = (0_isize, 0_isize);
    //
    // grid[r_s][c_s] = '#';
    let mut r = r_s;
    let mut c = c_s;

    let mut points = vec![(r_s, c_s)];

    for instr in instructions.iter() {
        match instr.dir {
            'U' => {
                for _ in 0..instr.distance {
                    r -= 1;
                }
                points.push((r, c));
            }
            'D' => {
                for _ in 0..instr.distance {
                    r += 1;
                }
                points.push((r, c));
            }
            'L' => {
                for _ in 0..instr.distance {
                    c -= 1;
                }
                points.push((r, c));
            }
            'R' => {
                for _ in 0..instr.distance {
                    c += 1;
                }
                points.push((r, c));
            }
            _ => unreachable!(),
        }
    }

    assert!((r, c) == (r_s, c_s));

    let boundary = instructions.iter().map(|i| i.distance).sum::<usize>();

    let mut area = 0_isize;

    points.windows(2).for_each(|pair| {
        let ((r1, c1), (r2, c2)) = (pair[0], pair[1]);
        area += r1 * c2 - r2 * c1;
    });

    area = area.abs() / 2;

    area as usize + boundary / 2 + 1
}

fn is_enclosed(grid: &[Vec<char>], (r, c): (usize, usize), debug: bool) -> bool {
    if c == 0 || c == grid[0].len() - 1 || r == 0 || r == grid.len() - 1 {
        return false;
    }

    let up = {
        let mut edges = 0;
        let mut prev_corner = None;

        for i in (0..r).rev() {
            if grid[i][c] == '#' {
                let left = grid[i][c - 1];
                let right = grid[i][c + 1];
                match (left, right) {
                    ('#', '#') => {
                        edges += 1;
                    }
                    ('#', '.') => match prev_corner {
                        Some('R') => edges += 1,
                        None => prev_corner = Some('L'),
                        _ => (),
                    },
                    ('.', '#') => match prev_corner {
                        Some('L') => edges += 1,
                        None => prev_corner = Some('R'),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        edges > 0 && edges % 2 == 1
    };

    if !debug && !up {
        return false;
    }

    let down = {
        let mut edges = 0;
        let mut prev_corner = None;

        for i in r + 1..grid.len() {
            if grid[i][c] == '#' {
                let left = grid[i][c - 1];
                let right = grid[i][c + 1];
                match (left, right) {
                    ('#', '#') => {
                        edges += 1;
                    }
                    ('#', '.') => match prev_corner {
                        Some('R') => edges += 1,
                        None => prev_corner = Some('L'),
                        _ => (),
                    },
                    ('.', '#') => match prev_corner {
                        Some('L') => edges += 1,
                        None => prev_corner = Some('R'),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        edges > 0 && edges % 2 == 1
    };

    if !debug && !down {
        return false;
    }

    let left = {
        let mut edges = 0;
        let mut prev_corner = None;

        for j in (0..c).rev() {
            if grid[r][j] == '#' {
                let top = grid[r - 1][j];
                let bottom = grid[r + 1][j];
                match (top, bottom) {
                    ('#', '#') => {
                        edges += 1;
                    }
                    ('#', '.') => match prev_corner {
                        Some('B') => edges += 1,
                        None => prev_corner = Some('T'),
                        _ => (),
                    },
                    ('.', '#') => match prev_corner {
                        Some('T') => edges += 1,
                        None => prev_corner = Some('B'),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        edges > 0 && edges % 2 == 1
    };

    if !debug && !left {
        return false;
    }

    let right = {
        let mut edges = 0;
        let mut prev_corner = None;

        for j in c + 1..grid[0].len() {
            if grid[r][j] == '#' {
                let top = grid[r - 1][j];
                let bottom = grid[r + 1][j];

                match (top, bottom) {
                    ('#', '#') => {
                        edges += 1;
                    }
                    ('#', '.') => match prev_corner {
                        Some('B') => edges += 1,
                        None => prev_corner = Some('T'),
                        _ => (),
                    },
                    ('.', '#') => match prev_corner {
                        Some('T') => edges += 1,
                        None => prev_corner = Some('B'),
                        _ => (),
                    },
                    _ => (),
                }
            }
        }

        if debug {
            dbg!(edges, prev_corner);
        }

        edges > 0 && edges % 2 == 1
    };

    if debug {
        dbg!(up, down, left, right);
    }

    right
}

#[test]
fn example() {
    let example: &str = r"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(part1(example), 62);
    assert_eq!(part2(example), 952408144115);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day18.txt");
    assert_eq!(part1(input), 36725);
    assert_eq!(part2(input), 97874103749720);
}
