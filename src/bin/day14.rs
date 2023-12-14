use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../../inputs/day14.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let N = grid.len();
    weigh_grid(&north(&grid))

    // let mut curr_rows = vec![N; grid[0].len()];
    // let mut weights = vec![0; grid[0].len()];
    // for (i, row) in grid.iter().enumerate() {
    //     for (j, ch) in row.iter().enumerate() {
    //         // if (i, j) == (3, 4) {
    //         //     dbg!(i, j);
    //         //     dbg!(ch);
    //         //     dbg!(curr_rows[j]);
    //         // }
    //         if *ch == 'O' && curr_rows[j] > 0 {
    //             // dbg!(i, j);
    //             // dbg!(curr_rows[j]);
    //             // dbg!(weights[j]);
    //             weights[j] += curr_rows[j];
    //             curr_rows[j] -= 1;
    //             // dbg!(curr_rows[j]);
    //             // dbg!(weights[j]);
    //         } else if *ch == '#' {
    //             curr_rows[j] = N - i - 1;
    //         }
    //     }
    // }
    //
    // dbg!(&weights);
    //
    // weights.iter().sum()
}

fn part2(input: &str) -> usize {
    const CYCLES: usize = 1000000000;

    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let N = grid.len();

    let mut i = 0;
    let mut prev = grid.clone();
    let mut prev_weights = vec![];
    let mut prev_weights_map = std::collections::HashMap::new();
    let mut cycle_index = None;
    while i < CYCLES {
        for _ in 0..4 {
            grid = transpose_clockwise(&north(&grid));
        }
        if grid
            .iter()
            .zip(prev.iter())
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(a, b)| a == b))
        {
            break;
        }
        // if grid == prev {
        //     break;
        // }
        prev = grid.clone();

        let grid_str = grid
            .iter()
            .map(|r| r.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("");
        prev_weights.push(weigh_grid(&grid));
        if let Some(cycle_start) = prev_weights_map.get(&grid_str) {
            let cycle_end = i;
            let cycle_len = cycle_end - cycle_start;
            dbg!(cycle_start, cycle_end, cycle_len);
            cycle_index = Some((CYCLES - cycle_start - 1) % cycle_len + cycle_start);
            dbg!(cycle_index);
            dbg!(prev_weights[cycle_index.unwrap()]);
            break;
        } else {
            prev_weights_map.insert(grid_str, i);
        }

        i += 1;
        // if i == 3 {
        //     dbg!(&grid);
        //     dbg!(weigh_grid(&grid));
        //     break;
        // }
    }

    prev_weights[cycle_index.unwrap()]
}

fn north(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; grid[0].len()]; grid.len()];

    let N = grid.len();
    let mut curr_rows = vec![N; grid[0].len()];
    // let mut weights = vec![0; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            // if (i, j) == (3, 4) {
            //     dbg!(i, j);
            //     dbg!(ch);
            //     dbg!(curr_rows[j]);
            // }
            if *ch == 'O' && curr_rows[j] > 0 {
                ret[N - curr_rows[j]][j] = 'O';
                // dbg!(i, j);
                // dbg!(curr_rows[j]);
                // dbg!(weights[j]);
                // weights[j] += curr_rows[j];
                curr_rows[j] -= 1;
                // dbg!(curr_rows[j]);
                // dbg!(weights[j]);
            } else if *ch == '#' {
                ret[i][j] = '#';
                curr_rows[j] = N - i - 1;
            }
        }
    }

    ret
}

fn weigh_grid(grid: &[Vec<char>]) -> usize {
    let N = grid.len();
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(_j, ch)| if *ch == 'O' { N - i } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn transpose_anticlockwise(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut ret = vec![vec!['.'; grid.len()]; grid[0].len()];
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '.' {
                ret[j][i] = *ch;
            }
        }
    }
    ret
}

fn transpose_clockwise(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let N = grid.len();
    let M = grid[0].len();
    let mut ret = vec![vec!['.'; N]; M];
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch != '.' {
                ret[j][N - i - 1] = *ch;
            }
        }
    }
    ret
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

// #[test]
// fn test_cases() {
//     let case = "#....#.
// ....#.#
// ...#...
// ...#...
// ....#.#
// #....#.
// ##.#.#.
// #.#.###
// #.####.
// #.#.#..
// #.#.#..
// #.####.
// #.#.##.
// ##.#.#.
// #....#.
// ....#.#
// ...#...";
//     let case_vec = case
//         .lines()
//         .map(|l| l.chars().collect::<Vec<_>>())
//         .collect::<Vec<_>>();
//
//     assert_eq!(compute(&case_vec, (None, None)), (Some(3), None));
//     assert_eq!(compute2(&case_vec), (Some(10), None));
//
//     assert_eq!(part1(case), 300);
//     assert_eq!(part2(case), 1000);
// }
//
#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day14.txt");
    assert_eq!(part1(input), 106990);
    // assert_eq!(part2(input), 39037);
}
