use std::collections::hash_map::{Entry, HashMap};
use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../../inputs/day22.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

type Coord = (usize, usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    lower: Coord,
    upper: Coord,
}

fn parse(input: &str) -> Vec<Brick> {
    // (lower, upper)
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('~').unwrap();

            let a = a
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let a = (a[0], a[1], a[2]);

            let b = b
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let b = (b[0], b[1], b[2]);

            if b.2 < a.2 {
                Brick { lower: b, upper: a }
            } else {
                Brick { lower: a, upper: b }
            }
        })
        .collect()
}

fn move_bricks(bricks_snapshot: &mut [Brick]) -> (&mut [Brick], usize) {
    let mut moving = vec![true; bricks_snapshot.len()];
    let mut moved = HashSet::new();

    let mut still_moving = moving.iter().any(|&b| b);
    while still_moving {
        for i in 0..bricks_snapshot.len() {
            if !moving[i] {
                continue;
            }

            let brick = bricks_snapshot[i].clone();
            if brick.lower.2 == 1 {
                moving[i] = false;
                continue;
            }

            // let mut moving = brick.lower.2 > 1;

            for j in 0..i {
                if moving[j] {
                    continue;
                }
                let other = &bricks_snapshot[j];
                if brick.lower.2 - 1 == other.upper.2 {
                    let ox = {
                        if other.lower.0 < other.upper.0 {
                            (other.lower.0, other.upper.0)
                        } else {
                            (other.upper.0, other.lower.0)
                        }
                    };
                    let oy = {
                        if other.lower.1 < other.upper.1 {
                            (other.lower.1, other.upper.1)
                        } else {
                            (other.upper.1, other.lower.1)
                        }
                    };
                    let bx = {
                        if brick.lower.0 < brick.upper.0 {
                            (brick.lower.0, brick.upper.0)
                        } else {
                            (brick.upper.0, brick.lower.0)
                        }
                    };
                    let by = {
                        if brick.lower.1 < brick.upper.1 {
                            (brick.lower.1, brick.upper.1)
                        } else {
                            (brick.upper.1, brick.lower.1)
                        }
                    };
                    let x_overlap = {
                        let tmp = (ox.0.max(bx.0), ox.1.min(bx.1));
                        tmp.0 <= tmp.1
                    };
                    let y_overlap = {
                        let tmp = (oy.0.max(by.0), oy.1.min(by.1));
                        tmp.0 <= tmp.1
                    };
                    if x_overlap && y_overlap && !moving[j] {
                        moving[i] = false;
                        break;
                    }
                }
            }
        }

        for i in 0..bricks_snapshot.len() {
            if moving[i] {
                let mut brick = bricks_snapshot[i];
                brick = Brick {
                    lower: (brick.lower.0, brick.lower.1, brick.lower.2 - 1),
                    upper: (brick.upper.0, brick.upper.1, brick.upper.2 - 1),
                };
                moved.insert(i);
                if brick.lower.2 == 1 {
                    moving[i] = false;
                }
                bricks_snapshot[i] = brick;
            }
        }
        still_moving = moving.iter().any(|&b| b);
    }

    (bricks_snapshot, moved.len())
}

fn check_bricks_support(bricks_stable: &[Brick]) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    // idx is supporting val_set
    let mut supporting = vec![HashSet::new(); bricks_stable.len()];

    // idx is supported by val_set
    let mut supported_by = vec![HashSet::new(); bricks_stable.len()];

    for i in 0..bricks_stable.len() {
        let brick = bricks_stable[i].clone();

        if brick.lower.2 == 1 {
            continue;
        }

        // let mut moving = brick.lower.2 > 1;

        for j in 0..i {
            let other = &bricks_stable[j];
            if brick.lower.2 - 1 == other.upper.2 {
                let ox = {
                    if other.lower.0 < other.upper.0 {
                        (other.lower.0, other.upper.0)
                    } else {
                        (other.upper.0, other.lower.0)
                    }
                };
                let oy = {
                    if other.lower.1 < other.upper.1 {
                        (other.lower.1, other.upper.1)
                    } else {
                        (other.upper.1, other.lower.1)
                    }
                };
                let bx = {
                    if brick.lower.0 < brick.upper.0 {
                        (brick.lower.0, brick.upper.0)
                    } else {
                        (brick.upper.0, brick.lower.0)
                    }
                };
                let by = {
                    if brick.lower.1 < brick.upper.1 {
                        (brick.lower.1, brick.upper.1)
                    } else {
                        (brick.upper.1, brick.lower.1)
                    }
                };
                let x_overlap = {
                    assert!(ox.0 <= ox.1);
                    assert!(bx.0 <= bx.1);

                    let tmp = (ox.0.max(bx.0), ox.1.min(bx.1));
                    tmp.0 <= tmp.1
                };
                let y_overlap = {
                    assert!(oy.0 <= oy.1);
                    assert!(by.0 <= by.1);
                    let tmp = (oy.0.max(by.0), oy.1.min(by.1));
                    tmp.0 <= tmp.1
                };
                if x_overlap && y_overlap {
                    supporting[j].insert(i);
                    supported_by[i].insert(j);
                }
            }
        }
    }

    // dbg!(&supporting);
    // dbg!(&supported_by);

    (supporting, supported_by)
}

fn part1(input: &str) -> usize {
    let mut bricks_snapshot = parse(input);
    bricks_snapshot.sort_by_key(|Brick { lower, upper }| (lower.2, upper.2));

    // println!("bricks sorted {:?}", &bricks);
    let (bricks_stable, _) = move_bricks(&mut bricks_snapshot);
    let (supporting, supported_by) = check_bricks_support(bricks_stable);

    (0..bricks_snapshot.len())
        .filter(|&i| {
            if supporting[i].is_empty() {
                true
            } else {
                supporting[i].iter().all(|&j| supported_by[j].len() > 1)
            }
        })
        .map(|i| {
            dbg!(&i);
            i
        })
        .count()
}

// brute force: runs in 30+ seconds
fn part2(input: &str) -> usize {
    let mut bricks_snapshot = parse(input);
    bricks_snapshot.sort_by_key(|Brick { lower, upper }| (lower.2, upper.2));

    // println!("bricks sorted {:?}", &bricks);
    let (bricks_stable, _) = move_bricks(&mut bricks_snapshot);
    let (supporting, supported_by) = check_bricks_support(bricks_stable);
    println!("supporting {:?}", &supporting);
    println!("supported_by {:?}", &supported_by);

    let mut ret = 0;

    (0..bricks_stable.len()).for_each(|i| {
        let mut rest = bricks_stable
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &b)| b)
            .collect::<Vec<Brick>>();
        let (_, moved) = move_bricks(&mut rest);
        ret += moved;
    });

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
    assert_eq!(part1(example), 5);
    assert_eq!(part2(example), 7);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day22.txt");
    assert_eq!(part1(input), 448);
    assert_eq!(part2(input), 57770);
}
