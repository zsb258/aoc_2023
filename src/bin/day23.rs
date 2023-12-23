use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("../../inputs/day23.txt");
    println!("Part1: {}", part1(input));
    // 3+ min
    println!("Part2: {}", part2(input));
}

type Coord = (usize, usize);

fn to_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn part1(input: &str) -> usize {
    let grid = to_grid(input);

    // assume start and end
    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let mut heap = BinaryHeap::new();
    let mut queue = VecDeque::new();
    let mut memo = HashMap::new();

    let mut ret = Vec::new();

    heap.push((0, start, (0, 0)));
    queue.push_back((0, start, (0, 0)));

    while let Some((cost, (r, c), dir)) = queue.pop_front() {
        if (r, c) == end {
            ret.push(cost);
        }
        if memo.get(&((r, c), dir)).is_some_and(|&v| cost < v) {
            continue;
        }

        let dirs = match grid[r][c] {
            '^' => vec![(-1, 0)],
            'v' => vec![(1, 0)],
            '<' => vec![(0, -1)],
            '>' => vec![(0, 1)],
            _ => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        };

        for (dr, dc) in dirs {
            if dir == (-dr, -dc) {
                continue;
            }
            let rr = (r as isize + dr) as usize;
            let cc = (c as isize + dc) as usize;
            if rr >= grid.len() || cc >= grid[0].len() {
                continue;
            }
            if grid[rr][cc] == '#' {
                continue;
            }
            if cost + 1 > *memo.get(&((rr, cc), (dr, dc))).unwrap_or(&0) {
                memo.insert(((rr, cc), (dr, dc)), cost + 1);
                queue.push_back((cost + 1, (rr, cc), (dr, dc)));
            }
        }
    }

    ret.into_iter().max().unwrap()
}

fn extract_nodes(
    grid: &[Vec<char>],
    end: Coord,
) -> (HashSet<Coord>, HashMap<(Coord, Coord), usize>) {
    let start = (0, 1);
    let start_dir = (1, 0);
    let start_next = (1, 1);

    let mut all_nodes: HashSet<Coord> = HashSet::new();
    let mut all_edges: HashMap<(Coord, Coord), usize> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_next, start_dir, start, 1));

    while let Some(((r, c), dir, prev_node, weight)) = queue.pop_front() {
        let key = if (r, c).cmp(&prev_node) == std::cmp::Ordering::Less {
            ((r, c), prev_node)
        } else {
            (prev_node, (r, c))
        };
        if all_edges.contains_key(&key) {
            continue;
        }

        let mut paths = Vec::new();
        let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dr, dc) in dirs {
            if dir == (-dr, -dc) {
                continue;
            }
            let rr = (r as isize + dr) as usize;
            let cc = (c as isize + dc) as usize;
            if rr >= grid.len() || cc >= grid[0].len() {
                continue;
            }
            if grid[rr][cc] == '#' {
                continue;
            }
            paths.push((dr, dc));
        }

        if paths.len() == 1 {
            let dir = paths[0];
            let rr = (r as isize + dir.0) as usize;
            let cc = (c as isize + dir.1) as usize;
            queue.push_back(((rr, cc), dir, prev_node, weight + 1));
        } else if paths.len() > 1 || (paths.is_empty() && (r, c) == end) {
            // junction
            all_nodes.insert((r, c));
            all_nodes.insert(prev_node);
            all_edges.insert(key, weight);

            for (dr, dc) in paths {
                let rr = (r as isize + dr) as usize;
                let cc = (c as isize + dc) as usize;
                queue.push_back(((rr, cc), (dr, dc), (r, c), 1));
            }
        }
    }

    (all_nodes, all_edges)
}

fn part2(input: &str) -> usize {
    let grid = to_grid(input);

    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let (nodes, edges) = extract_nodes(&grid, end);

    let nodes = nodes
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let start_edges = edges
        .iter()
        .filter(|(&(l, r), _v)| start == l || start == r);

    let mut memo = HashMap::new();
    let mut heap = BinaryHeap::new();
    for (&key, &weight) in start_edges {
        let mask = 1_usize << nodes.get(&start).unwrap();
        let next = if key.0 == start { key.1 } else { key.0 };
        heap.push((weight, next, mask));
    }

    let mut ret = 0;

    while let Some((cost, curr, nodes_seen)) = heap.pop() {
        if nodes.contains_key(&curr) && nodes_seen & (1 << *nodes.get(&curr).unwrap()) != 0 {
            continue;
        }
        if curr == end {
            ret = ret.max(cost);
        }
        if memo.get(&(curr, nodes_seen)).is_some_and(|&v| cost < v) {
            continue;
        }

        let outgoing = edges
            .iter()
            .filter(|(&(first, second), _v)| curr == first || curr == second);

        for (key, weight) in outgoing {
            let next = if key.0 == curr { key.1 } else { key.0 };
            let nodes_seen_next = nodes_seen | (1 << *nodes.get(&curr).unwrap());
            if cost + weight > *memo.get(&(next, nodes_seen_next)).unwrap_or(&0) {
                heap.push((cost + weight, next, nodes_seen_next));
                memo.insert((next, nodes_seen_next), cost + weight);
            }
        }
    }

    ret
}

#[test]
fn example() {
    let example: &str = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    assert_eq!(part1(example), 94);
    assert_eq!(part2(example), 154);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day23.txt");
    assert_eq!(part1(input), 2010);
    assert_eq!(part2(input), 6318);
}
