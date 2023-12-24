use itertools::Itertools;
use z3::ast::{Ast, Int};

fn main() {
    let input: &str = include_str!("../../inputs/day24.txt");
    println!("Part1: {}", part1(input, (2e14, 4e14)));
    println!("Part2: {}", part2(input));
}

type Coord = (f64, f64, f64);

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    pos: Coord,
    vel: Coord,
}

fn parse(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line
                .split(" @ ")
                .map(|part| {
                    part.split(", ")
                        .map(|num| num.trim().parse::<f64>().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap();
            Hailstone { pos, vel }
        })
        .collect_vec()
}

fn part1(input: &str, (lbound, ubound): (f64, f64)) -> usize {
    let segments = parse(input)
        .into_iter()
        .map(|h| {
            // min and max t to be within box
            let mut t_lower = 0_f64;
            let mut t_upper = f64::MAX;

            for (p, v) in [(h.pos.0, h.vel.0), (h.pos.1, h.vel.1)] {
                match (p > lbound, p < ubound, v < 0_f64) {
                    (true, true, true) => {
                        // increase towards lbound
                        t_upper = t_upper.min((lbound - p) / v);
                    }
                    (true, true, false) => {
                        // decrease towards ubound
                        t_upper = t_upper.min((ubound - p) / v);
                    }
                    (false, true, false) => {
                        t_lower = t_lower.max((lbound - p) / v);
                        t_upper = t_upper.min((ubound - p) / v);
                    }
                    (true, false, true) => {
                        t_lower = t_lower.max((ubound - p) / v);
                        t_upper = t_upper.min((lbound - p) / v);
                    }
                    _ => {}
                }
            }

            let x_segment = (h.pos.0 + h.vel.0 * t_lower, h.pos.0 + h.vel.0 * t_upper);

            let y_segment = (h.pos.1 + h.vel.1 * t_lower, h.pos.1 + h.vel.1 * t_upper);

            assert!(x_segment.0.round() >= lbound && x_segment.0.round() <= ubound);
            assert!(x_segment.1.round() >= lbound && x_segment.1.round() <= ubound);
            assert!(y_segment.0.round() >= lbound && y_segment.0.round() <= ubound);
            assert!(y_segment.1.round() >= lbound && y_segment.1.round() <= ubound);

            (x_segment, y_segment)
        })
        .collect_vec();

    let mut count = 0;

    for i in 0..(segments.len() - 1) {
        for j in (i + 1)..segments.len() {
            let ((x1, x2), (y1, y2)) = segments[i];
            let ((x3, x4), (y3, y4)) = segments[j];

            let t1 = (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1);
            let t2 = (x4 - x1) * (y2 - y1) - (y4 - y1) * (x2 - x1);
            let t3 = (x1 - x3) * (y4 - y3) - (y1 - y3) * (x4 - x3);
            let t4 = (x2 - x3) * (y4 - y3) - (y2 - y3) * (x4 - x3);

            if t1 * t2 < 0_f64 && t3 * t4 < 0_f64 {
                count += 1;
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    // https://github.com/prove-rs/z3.rs/tree/master/z3
    // https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepwd37/?utm_name=web3xcss
    // https://gist.github.com/WaterFace/1240609d0d4e15fa4ade3e471e7b501e
    let hails = parse(input);
    let hails_to_use = hails.iter().take(3);

    let cfg = z3::Config::new();
    let context = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&context);

    let x = Int::new_const(&context, "x");
    let y = Int::new_const(&context, "y");
    let z = Int::new_const(&context, "z");
    let vx = Int::new_const(&context, "vx");
    let vy = Int::new_const(&context, "vy");
    let vz = Int::new_const(&context, "vz");

    for (i, h) in hails_to_use.enumerate() {
        let xi = Int::from_i64(&context, h.pos.0 as i64);
        let vxi = Int::from_i64(&context, h.vel.0 as i64);
        let yi = Int::from_i64(&context, h.pos.1 as i64);
        let vyi = Int::from_i64(&context, h.vel.1 as i64);
        let zi = Int::from_i64(&context, h.pos.2 as i64);
        let vzi = Int::from_i64(&context, h.vel.2 as i64);

        let ti = Int::new_const(&context, format!("t{i}"));
        solver.assert(&ti.gt(&Int::from_i64(&context, 0)));

        solver.assert(&(x.clone() + vx.clone() * ti.clone())._eq(&(xi + vxi * ti.clone())));
        solver.assert(&(y.clone() + vy.clone() * ti.clone())._eq(&(yi + vyi * ti.clone())));
        solver.assert(&(z.clone() + vz.clone() * ti.clone())._eq(&(zi + vzi * ti.clone())));
    }

    if let z3::SatResult::Sat = solver.check() {
        if let Some(m) = solver.get_model() {
            let x = m.eval(&x, true).unwrap().as_i64().unwrap() as usize;
            let y = m.eval(&y, true).unwrap().as_i64().unwrap() as usize;
            let z = m.eval(&z, true).unwrap().as_i64().unwrap() as usize;

            return x + y + z;
        }
    }

    panic!("no solution");
}

#[test]
fn example() {
    let example: &str = r"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(part1(example, (7_f64, 27_f64)), 2);
    assert_eq!(part2(example), 47);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day24.txt");
    assert_eq!(part1(input, (2e14, 4e14)), 25433);
    assert_eq!(part2(input), 885093461440405);
}
