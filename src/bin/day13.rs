fn main() {
    let input: &str = include_str!("../../inputs/day13.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn solve_with(
    input: &str,
    solver: &dyn Fn(Vec<Vec<char>>) -> (Option<usize>, Option<usize>),
) -> usize {
    input
        .split("\n\n")
        .map(|raw| {
            raw.lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(solver)
        .fold(0, |acc, (row, col)| match (row, col) {
            (Some(r), None) => acc + r * 100,
            (None, Some(c)) => acc + c,
            _ => unreachable!(),
        })
}

fn part1(input: &str) -> usize {
    solve_with(input, &|grid| compute(&grid, (None, None)))
}

fn part2(input: &str) -> usize {
    solve_with(input, &|grid| compute2(&grid))
}

fn compute(
    grid: &[Vec<char>],
    original: (Option<usize>, Option<usize>),
) -> (Option<usize>, Option<usize>) {
    let row = (1..grid.len()).find(|&i| {
        let ok = (0..(i.min(grid.len() - i))).all(|di| {
            grid[i - di - 1]
                .iter()
                .zip(grid[i + di].iter())
                .all(|(a, b)| a == b)
        });
        if ok {
            if let Some(r) = original.0 {
                if r != i {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    });

    let col = (1..grid[0].len()).find(|&j| {
        let ok = (0..(j.min(grid[0].len() - j)))
            .all(|dj| grid.iter().all(|row| row[j - dj - 1] == row[j + dj]));
        if ok {
            if let Some(c) = original.1 {
                if c != j {
                    return true;
                }
            } else {
                return true;
            }
        }

        false
    });

    (row, col)
}

fn compute2(grid: &[Vec<char>]) -> (Option<usize>, Option<usize>) {
    let original = compute(grid, (None, None));
    let mut new_grid = grid.to_owned().to_vec();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[i][j] = match grid[i][j] {
                '.' => '#',
                '#' => '.',
                _ => unreachable!(),
            };

            match compute(&new_grid, original) {
                (Some(r), None) => return (Some(r), None),
                (None, Some(c)) => return (None, Some(c)),
                (None, None) => (),
                (Some(_), Some(_)) => unreachable!(),
            }

            new_grid[i][j] = grid[i][j];
        }
    }

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
