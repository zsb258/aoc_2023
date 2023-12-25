use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("../../inputs/day02.txt");
    println!("Part1: {}", solve_with(input, part1));
    println!("Part2: {}", solve_with(input, part2));
}

fn solve_with(input: &str, solver: impl Fn(Game) -> Option<u32>) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let game = parse_line(line);
            solver(game)
        })
        .sum()
}

fn part1(game: Game) -> Option<u32> {
    const MAX_R: u32 = 12;
    const MAX_G: u32 = 13;
    const MAX_B: u32 = 14;

    if game.r <= MAX_R && game.g <= MAX_G && game.b <= MAX_B {
        Some(game.id)
    } else {
        None
    }
}

fn part2(game: Game) -> Option<u32> {
    Some(game.r * game.g * game.b)
}

struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32,
}

fn parse_line(line: &str) -> Game {
    let (id_str, draws_str) = match line.splitn(2, ": ").collect::<Vec<_>>()[..] {
        [id_str, draws_str] => (id_str, draws_str),
        _ => panic!("Invalid line format"),
    };

    let id = id_str.split(' ').last().unwrap().parse::<u32>().unwrap();

    let draws = draws_str
        .split("; ")
        .map(|draw_str| {
            draw_str
                .split(", ")
                .map(|draw| match draw.splitn(2, ' ').collect::<Vec<_>>()[..] {
                    [count, color] => {
                        (color.chars().next().unwrap(), count.parse::<u32>().unwrap())
                    }
                    _ => panic!("Invalid draw format"),
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let mut max_vals = ['r', 'g', 'b'].iter().map(|color| {
        draws
            .iter()
            .map(|draw| *draw.get(color).unwrap_or(&0))
            .max()
            .unwrap()
    });

    Game {
        id,
        r: max_vals.next().unwrap(),
        g: max_vals.next().unwrap(),
        b: max_vals.next().unwrap(),
    }
}

#[test]
fn example() {
    let example: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(solve_with(example, part1), 8);
    assert_eq!(solve_with(example, part2), 2286);
}
