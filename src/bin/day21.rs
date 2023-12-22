use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("../../inputs/day21.txt");
    println!("Part1: {}", part1(input, 64));
    println!("Part2: {}", part2(input, 26501365));
}

fn to_grid(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            if let Some((c, _)) = row.iter().enumerate().find(|(_, ch)| **ch == 'S') {
                Some((r, c))
            } else {
                None
            }
        })
        .next()
        .unwrap();

    (grid, start)
}

fn brute_force(grid: &[Vec<char>], (r_s, c_s): (usize, usize), targets: &[usize]) -> Vec<usize> {
    let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::new();
    let mut visited: [HashSet<(isize, isize)>; 2] = [HashSet::new(), HashSet::new()];

    let mut target_i = 0;
    let mut ret = Vec::new();

    queue.push_back(((r_s as isize, c_s as isize), 0));

    while let Some(((r, c), dist)) = queue.pop_front() {
        if dist > targets[target_i] {
            ret.push(visited[(targets[target_i]) % 2].len());

            target_i += 1;
            if target_i >= targets.len() {
                break;
            }
        }

        if visited.iter().any(|set| set.contains(&(r, c))) {
            continue;
        }
        visited[dist % 2].insert((r, c));

        for (dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (rr, cc) = ((r + dr), (c + dc));
            let r_norm = rr.rem_euclid(grid.len() as isize) as usize;
            let c_norm = cc.rem_euclid(grid[0].len() as isize) as usize;

            if grid[r_norm][c_norm] == '#' {
                continue;
            }
            queue.push_back(((rr, cc), dist + 1));
        }
    }

    ret
}

fn part1(input: &str, maxstep: usize) -> usize {
    let (grid, start) = to_grid(input);
    brute_force(&grid, start, &[maxstep])[0]
}

/// the interpolation method only works for the given input with special format
fn part2(input: &str, maxstep: usize) -> usize {
    let (grid, (r_s, c_s)) = to_grid(input);

    // needs at least 3 points
    // though using more points does not seem to change the result of regression
    let x_data_len = 5;
    let min_x = maxstep % grid.len();
    let max_x = min_x + grid.len() * (x_data_len - 1);

    let x_data = (min_x..=max_x).step_by(grid.len()).collect_vec();
    let y_data = brute_force(&grid, (r_s, c_s), &x_data);

    let ret_regression = regression_interpolate(maxstep, &x_data, &y_data);

    let ret_lagrange = lagrange_interpolate(maxstep, &x_data, &y_data);

    let ret_magic = magic_interpolate(maxstep / grid.len(), &x_data, &y_data);

    assert!(ret_regression == ret_lagrange && ret_lagrange == ret_magic);

    ret_regression
}

fn regression_interpolate(x: usize, x_data: &[usize], y_data: &[usize]) -> usize {
    assert!(x_data.len() == y_data.len());

    #[allow(unused_imports)]
    use nalgebra::{DMatrix, Dyn, Matrix, MatrixXx1, VecStorage, U1};

    type MatA = DMatrix<f64>;
    type MatB = MatrixXx1<f64>;

    // full types for above
    // type MatA = Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>>;
    // type MatB = Matrix<f64, Dyn, U1, VecStorage<f64, Dyn, U1>>;

    let mat_a: MatA = MatA::from_row_slice(
        x_data.len(),
        3,
        x_data
            .iter()
            .flat_map(|x| {
                let x = *x as f64;
                [x * x, x, 1_f64]
            })
            .collect::<Vec<_>>()
            .as_slice(),
    );

    let mat_b: MatB =
        MatB::from_column_slice(y_data.iter().map(|x| *x as f64).collect_vec().as_slice());

    // seems like the library can only solve non-square matrices by svd
    // I don't know enough about the various methods so just used whichever that worked
    let res = mat_a.svd(true, true).solve(&mat_b, 1e-7).unwrap();

    match res.data.as_slice() {
        &[a, b, c] => {
            let x = x as f64;
            (a * x * x + b * x + c).round() as usize
        }
        _ => unreachable!(),
    }
}

fn lagrange_interpolate(x: usize, x_data: &[usize], y_data: &[usize]) -> usize {
    let (x1, x2, x3) = match x_data[..3] {
        [x1, x2, x3] => (x1 as isize, x2 as isize, x3 as isize),
        _ => unreachable!(),
    };
    let (y1, y2, y3) = match y_data[..3] {
        [y1, y2, y3] => (y1 as isize, y2 as isize, y3 as isize),
        _ => unreachable!(),
    };

    let x = x as isize;

    let term1 = y1 * ((x - x2) * (x - x3) / ((x1 - x2) * (x1 - x3)));
    let term2 = y2 * ((x - x1) * (x - x3) / ((x2 - x1) * (x2 - x3)));
    let term3 = y3 * ((x - x1) * (x - x2) / ((x3 - x1) * (x3 - x2)));

    (term1 + term2 + term3) as usize
}

fn magic_interpolate(x: usize, _x_data: &[usize], y_data: &[usize]) -> usize {
    // saw this on reddit; don't know what it is called
    // x is maxstep / grid.len(); different from the other two

    let a = y_data[0];
    let b = y_data[1];
    let c = y_data[2];
    a + x * (b - a) + x * (x - 1) / 2 * (c - 2 * b + a)
}

#[test]
fn example() {
    let example: &str = r"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    assert_eq!(part1(example, 6), 16);

    // brute force the part2 examples
    // takes 2+ min on my machine
    let (grid, start) = to_grid(example);
    let in_vec = vec![6, 10, 50, 100, 500, 1000, 5000];
    let expect_vec = vec![16, 50, 1594, 6536, 167004, 668697, 16733044];
    assert_eq!(brute_force(&grid, start, &in_vec), expect_vec);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day21.txt");
    assert_eq!(part1(input, 64), 3751);
    assert_eq!(part2(input, 26501365), 619407349431167);
}
