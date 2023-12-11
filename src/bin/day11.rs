fn main() {
    let input: &str = include_str!("../../inputs/day11.txt");
    println!("Part1: {}", solve(input, 2));
    println!("Part2: {}", solve(input, 1000000));
}

fn solve(input: &str, expansion_factor: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (rows_to_expand, cols_to_expand) = expand(&grid);

    let galaxies = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, ch)| if *ch == '#' { Some((r, c)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    let mut sum = 0;

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            // r2 > r1
            let (r1, r2) = if galaxies[i].0 < galaxies[j].0 {
                (galaxies[i].0, galaxies[j].0)
            } else {
                (galaxies[j].0, galaxies[i].0)
            };

            // c2 > c1
            let (c1, c2) = if galaxies[i].1 < galaxies[j].1 {
                (galaxies[i].1, galaxies[j].1)
            } else {
                (galaxies[j].1, galaxies[i].1)
            };

            let x = r2 - r1
                + ((r1..r2).filter(|r| rows_to_expand.contains(r)).count()
                    * (expansion_factor - 1));
            let y = c2 - c1
                + ((c1..c2).filter(|c| cols_to_expand.contains(c)).count()
                    * (expansion_factor - 1));

            sum += x + y;
        }
    }

    sum
}

fn expand(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let rows_to_expand = grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            if row.iter().all(|&ch| ch == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    let cols_to_expand = (0..grid.len())
        .filter(|c| (0..grid[0].len()).all(|r| grid[r][*c] == '.'))
        .collect::<Vec<usize>>();

    (rows_to_expand, cols_to_expand)
}

#[test]
fn example() {
    let example: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(solve(example, 2), 374);
    assert_eq!(solve(example, 10), 1030);
    assert_eq!(solve(example, 100), 8410);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day11.txt");
    assert_eq!(solve(input, 2), 10289334);
    assert_eq!(solve(input, 1000000), 649862989626);
}
