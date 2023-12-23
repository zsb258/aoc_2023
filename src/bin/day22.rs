use std::collections::{HashMap, HashSet};

fn main() {
    let input: &str = include_str!("../../inputs/day22.txt");
    let (bricks_stable, mut overlap_memo) = preprocess(input);
    println!("Part1: {}", part1(&bricks_stable, &mut overlap_memo));
    // brute force: runs in 30 seconds
    println!("Part2: {}", part2(&bricks_stable, &mut overlap_memo));
}

type Coord = (usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    lower: Coord,
    upper: Coord,
}

fn parse(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let coords = line
                .splitn(2, '~')
                .map(|s| {
                    match s
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()[..]
                    {
                        [x, y, z] => (x, y, z),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>();

            if coords[0].2 < coords[1].2 {
                Brick {
                    lower: coords[0],
                    upper: coords[1],
                }
            } else {
                Brick {
                    lower: coords[1],
                    upper: coords[0],
                }
            }
        })
        .collect()
}

fn preprocess(input: &str) -> (Vec<Brick>, HashMap<(usize, usize), bool>) {
    let mut bricks_snapshot = parse(input);
    bricks_snapshot.sort_by_key(|Brick { lower, upper }| (lower.2, upper.2));

    let mut overlap_memo = HashMap::new();
    let (bricks_stable, _) = freefall(&mut bricks_snapshot, &mut overlap_memo, None);

    (bricks_stable, overlap_memo)
}

fn is_xy_overlap(brick: &Brick, other: &Brick) -> bool {
    let x_overlap = {
        let ox = if other.lower.0 < other.upper.0 {
            (other.lower.0, other.upper.0)
        } else {
            (other.upper.0, other.lower.0)
        };
        let bx = if brick.lower.0 < brick.upper.0 {
            (brick.lower.0, brick.upper.0)
        } else {
            (brick.upper.0, brick.lower.0)
        };

        ox.0.max(bx.0) <= ox.1.min(bx.1)
    };

    let y_overlap = {
        let oy = if other.lower.1 < other.upper.1 {
            (other.lower.1, other.upper.1)
        } else {
            (other.upper.1, other.lower.1)
        };
        let by = if brick.lower.1 < brick.upper.1 {
            (brick.lower.1, brick.upper.1)
        } else {
            (brick.upper.1, brick.lower.1)
        };

        oy.0.max(by.0) <= oy.1.min(by.1)
    };

    x_overlap && y_overlap
}

/// returns bricks in stable positions and the number of bricks that moved
fn freefall(
    bricks: &mut [Brick],
    overlap_memo: &mut HashMap<(usize, usize), bool>,
    skip_idx: Option<usize>,
) -> (Vec<Brick>, usize) {
    let mut moving = vec![true; bricks.len()];
    let mut moved = HashSet::new();

    if let Some(i) = skip_idx {
        moving[i] = false;
    }

    let mut still_moving = moving.iter().any(|&b| b);
    while still_moving {
        for i in 0..bricks.len() {
            if !moving[i] {
                continue;
            }

            let brick = &bricks[i];
            if brick.lower.2 == 1 {
                moving[i] = false;
                continue;
            }

            for j in 0..i {
                if moving[j] {
                    continue;
                }
                match skip_idx {
                    Some(idx) if idx == j => continue,
                    _ => (),
                }

                let other = &bricks[j];
                if brick.lower.2 - 1 == other.upper.2
                    && *overlap_memo
                        .entry((i, j))
                        .or_insert(is_xy_overlap(brick, other))
                {
                    moving[i] = false;
                    break;
                }
            }
        }

        for i in 0..bricks.len() {
            if moving[i] {
                let mut b = bricks[i];
                b = Brick {
                    lower: (b.lower.0, b.lower.1, b.lower.2 - 1),
                    upper: (b.upper.0, b.upper.1, b.upper.2 - 1),
                };
                moved.insert(i);
                if b.lower.2 == 1 {
                    moving[i] = false;
                }
                bricks[i] = b;
            }
        }
        still_moving = moving.iter().any(|&b| b);
    }

    (bricks.into(), moved.len())
}

fn check_bricks_support(
    bricks_stable: &[Brick],
    overlap_memo: &mut HashMap<(usize, usize), bool>,
) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    // idx is supporting val_set
    let mut supporting = vec![HashSet::new(); bricks_stable.len()];

    // idx is supported by val_set
    let mut supported_by = vec![HashSet::new(); bricks_stable.len()];

    for i in 0..bricks_stable.len() {
        let brick = &bricks_stable[i];

        if brick.lower.2 == 1 {
            continue;
        }

        for j in 0..i {
            let other = &bricks_stable[j];
            if brick.lower.2 - 1 == other.upper.2
                && *overlap_memo
                    .entry((i, j))
                    .or_insert(is_xy_overlap(brick, other))
            {
                supporting[j].insert(i);
                supported_by[i].insert(j);
            }
        }
    }

    (supporting, supported_by)
}

fn part1(bricks_stable: &[Brick], overlap_memo: &mut HashMap<(usize, usize), bool>) -> usize {
    let (supporting, supported_by) = check_bricks_support(bricks_stable, overlap_memo);

    (0..bricks_stable.len())
        .filter(|&i| {
            if supporting[i].is_empty() {
                true
            } else {
                supporting[i].iter().all(|&j| supported_by[j].len() > 1)
            }
        })
        .count()
}

fn part2(bricks_stable: &[Brick], overlap_memo: &mut HashMap<(usize, usize), bool>) -> usize {
    let mut ret = 0;

    let bricks_stable = bricks_stable.to_vec();

    for i in 0..bricks_stable.len() {
        let (_, moved) = freefall(&mut bricks_stable.clone(), overlap_memo, Some(i));
        ret += moved;
    }

    ret
}

#[test]
fn example() {
    let example: &str = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    let (bricks_stable, mut overlap_memo) = preprocess(example);

    assert_eq!(part1(&bricks_stable, &mut overlap_memo), 5);
    assert_eq!(part2(&bricks_stable, &mut overlap_memo), 7);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day22.txt");

    let (bricks_stable, mut overlap_memo) = preprocess(input);

    assert_eq!(part1(&bricks_stable, &mut overlap_memo), 448);
    assert_eq!(part2(&bricks_stable, &mut overlap_memo), 57770);
}
