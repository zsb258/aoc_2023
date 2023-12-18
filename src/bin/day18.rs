fn main() {
    let input: &str = include_str!("../../inputs/day18.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

struct Instruction {
    dir: char,
    dist: isize,
}

fn part1(input: &str) -> usize {
    let instructions = input.lines().map(|line| {
        let mut splits = line.split_whitespace();
        Instruction {
            dir: splits.next().unwrap().chars().next().unwrap(),
            dist: splits.next().unwrap().parse::<isize>().unwrap(),
        }
    });

    get_area(instructions) as usize
}

fn part2(input: &str) -> usize {
    let instructions = input.lines().map(|line| {
        let hex = line.split_whitespace().last().unwrap();

        let dist = isize::from_str_radix(&hex[2..7], 16).unwrap();
        let dir = match &hex[7..].chars().next().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            _ => unreachable!(),
        };

        Instruction { dir, dist }
    });

    get_area(instructions) as usize
}

fn get_area(instructions: impl Iterator<Item = Instruction>) -> isize {
    struct Container {
        prev: (isize, isize),
        boundary_area: isize,
        shoelace_area: isize,
    }

    let Container {
        boundary_area,
        shoelace_area,
        ..
    } = instructions.fold(
        Container {
            prev: (0, 0),
            boundary_area: 0,
            shoelace_area: 0,
        },
        |mut acc, instr| {
            acc.boundary_area += instr.dist;
            let (mut r, mut c) = acc.prev;
            match instr.dir {
                'U' => r -= instr.dist,
                'D' => r += instr.dist,
                'L' => c -= instr.dist,
                'R' => c += instr.dist,
                _ => unreachable!(),
            }
            acc.shoelace_area += acc.prev.0 * c - r * acc.prev.1;
            acc.prev = (r, c);

            acc
        },
    );

    shoelace_area.abs() / 2 + boundary_area / 2 + 1
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
