fn main() {
    let input: &str = include_str!("../../inputs/day14.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> usize {
    weigh_grid(&north(&parse(input)))
}

fn part2(input: &str) -> usize {
    const CYCLES: usize = 1000000000;

    let mut i = 0;
    let mut prev_string = input.to_string();
    let mut history = std::collections::HashMap::new();

    let entry = loop {
        let grid_str = (0..4)
            .fold(parse(&prev_string), |acc, _| {
                transpose_clockwise(&north(&acc))
            })
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(cycle_start) = history.get(&grid_str) {
            let cycle_len = i - cycle_start;
            let cycle_index = (CYCLES - cycle_start - 1) % cycle_len + cycle_start;

            break history
                .into_iter()
                .find(|(_, v)| *v == cycle_index)
                .unwrap()
                .0;
        } else {
            prev_string = grid_str.clone();
            history.insert(grid_str, i);
        }

        i += 1;
    };

    weigh_grid(&parse(&entry))
}

fn north(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; grid[0].len()]; grid.len()];

    (0..grid[0].len()).for_each(|j| {
        let mut curr_row = 0;
        (0..grid.len()).for_each(|i| match grid[i][j] {
            'O' => {
                ret[curr_row][j] = 'O';
                curr_row += 1;
            }
            '#' => {
                ret[i][j] = '#';
                curr_row = i + 1;
            }
            _ => (),
        });
    });

    ret
}

fn weigh_grid(grid: &[Vec<char>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|ch| **ch == 'O').count() * (grid.len() - i))
        .sum::<usize>()
}

fn transpose_clockwise(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).rev().collect())
        .collect()
}

#[test]
fn example() {
    let example: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part1(example), 136);
    assert_eq!(part2(example), 64);
}

#[test]
fn test_case() {
    let example: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#O...";
    assert_eq!(part1(example), 137);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day14.txt");
    assert_eq!(part1(input), 106990);
    assert_eq!(part2(input), 100531);
}
