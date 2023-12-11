fn main() {
    let input: &str = include_str!("../../inputs/day11.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
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

            let x = r2 - r1 + (r1..r2).filter(|r| rows_to_expand.contains(r)).count();
            let y = c2 - c1 + (c1..c2).filter(|c| cols_to_expand.contains(c)).count();

            sum += x + y;
        }
    }

    sum
}

fn part2(input: &str) -> usize {
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

            let x =
                r2 - r1 + ((r1..r2).filter(|r| rows_to_expand.contains(r)).count() * (1000000 - 1));
            let y =
                c2 - c1 + ((c1..c2).filter(|c| cols_to_expand.contains(c)).count() * (1000000 - 1));

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
        .filter_map(|c| {
            if (0..grid[0].len()).all(|r| grid[r][c] == '.') {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    (rows_to_expand, cols_to_expand)
}

#[test]
fn part1_example() {
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
    assert_eq!(part1(example), 374);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day11.txt");
    assert_eq!(part1(input), 10289334);
    // assert_eq!(part2(input), 483);
}
