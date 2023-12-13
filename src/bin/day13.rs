fn main() {
    let input: &str = include_str!("../../inputs/day13.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|grid| compute(&grid, (None, None)))
        .fold(0, |acc, (row, col)| match (row, col) {
            (Some(r), None) => acc + r * 100,
            (None, Some(c)) => acc + c,
            _ => unreachable!(),
        })
}

fn compute(
    grid: &[Vec<char>],
    original: (Option<usize>, Option<usize>),
) -> (Option<usize>, Option<usize>) {
    // dbg!(grid);
    let mut row: Option<usize> = None;
    for i in 1..grid.len() {
        let mut ok = true;
        let mut t = i - 1;
        let mut b = i;
        while t >= 0 && b < grid.len() {
            let flag = grid[t].iter().zip(grid[b].iter()).all(|(a, b)| a == b);
            if flag {
                if t == 0 {
                    break;
                }
                t -= 1;
                b += 1;
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            if let Some(r) = original.0 {
                if r != i {
                    row = Some(i);
                    break;
                }
            } else {
                row = Some(i);
                break;
            }
        }
    }

    let mut col: Option<usize> = None;
    for j in 1..grid[0].len() {
        let mut ok = true;
        let mut l = j - 1;
        let mut r = j;
        while l >= 0 && r < grid[0].len() {
            let flag = grid.iter().all(|row| row[l] == row[r]);
            if flag {
                if l == 0 {
                    break;
                }
                l -= 1;
                r += 1;
            } else {
                ok = false;
                break;
            }
        }
        if ok {
            if let Some(c) = original.1 {
                if c != j {
                    col = Some(j);
                    break;
                }
            } else {
                col = Some(j);
                break;
            }
        }
    }

    // dbg!(row, col);

    (row, col)
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(|grid| {
            let tmp = compute2(&grid);
            dbg!(&tmp);
            tmp
        })
        .fold(0, |acc, (row, col)| match (row, col) {
            (Some(r), None) => acc + r * 100,
            (None, Some(c)) => acc + c,
            _ => unreachable!(),
        })
}

fn compute2(grid: &[Vec<char>]) -> (Option<usize>, Option<usize>) {
    let original = compute(grid, (None, None));
    dbg!(&original);
    let mut new_grid = grid.to_owned().to_vec();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[i][j] = match grid[i][j] {
                '.' => '#',
                '#' => '.',
                _ => unreachable!(),
            };

            if i == 12 && j == 6 {
                dbg!(&new_grid);
                dbg!(compute(&new_grid, original));
            }

            match compute(&new_grid, original) {
                (Some(r), Some(c)) => match original {
                    (Some(ori_r), None) => {
                        dbg!((i, j));
                        if ori_r == r {
                            return (None, Some(c));
                        } else {
                            return (Some(r), None);
                        }
                    }
                    (None, Some(ori_c)) => {
                        dbg!((i, j));
                        if ori_c == c {
                            return (Some(r), None);
                        } else {
                            return (None, Some(c));
                        }
                    }
                    _ => unreachable!(),
                },
                (Some(r), None) => {
                    if let Some(ori_r) = original.0 {
                        if ori_r != r {
                            dbg!((i, j));
                            return (Some(r), None);
                        }
                    } else {
                        dbg!((i, j));
                        return (Some(r), None);
                    }
                }
                (None, Some(c)) => {
                    if let Some(ori_c) = original.1 {
                        if ori_c != c {
                            dbg!((i, j));
                            return (None, Some(c));
                        }
                    } else {
                        dbg!((i, j));
                        return (None, Some(c));
                    }
                }
                (None, None) => (),
            }

            new_grid[i][j] = grid[i][j];
        }
    }

    dbg!(&new_grid);

    unreachable!()
}

#[test]
fn example() {
    let example: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!(part1(example), 405);
    assert_eq!(part2(example), 400);
}

#[test]
fn test_cases() {
    let case = "#....#.
....#.#
...#...
...#...
....#.#
#....#.
##.#.#.
#.#.###
#.####.
#.#.#..
#.#.#..
#.####.
#.#.##.
##.#.#.
#....#.
....#.#
...#...";
    let case_vec = case
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(compute(&case_vec, (None, None)), (Some(3), None));
    assert_eq!(compute2(&case_vec), (Some(10), None));

    assert_eq!(part1(case), 300);
    assert_eq!(part2(case), 1000);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day13.txt");
    assert_eq!(part1(input), 35691);
    assert_eq!(part2(input), 39037);
}
