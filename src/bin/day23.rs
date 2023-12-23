use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("../../inputs/day23.txt");
    println!("Part1: {}", part1(input));
    // 3+ min
    println!("Part2: {}", part2(input));
}

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

    // let max_cost = grid.len() * grid[0].len(); // step on all cells

    let mut heap = BinaryHeap::new();
    let mut queue = VecDeque::new();
    // let mut seen = HashSet::new();
    let mut memo = HashMap::new();

    let mut ret = Vec::new();

    heap.push((0, start, (0, 0)));
    queue.push_back((0, start, (0, 0)));

    // while let Some((cost, (r, c), dir)) = heap.pop() {
    while let Some((cost, (r, c), dir)) = queue.pop_front() {
        if (r, c) == end {
            // dbg!(&heap);
            // memo.iter().sorted().for_each(|(k, v)| {
            //     println!("{:?}: {}", k, v);
            // });
            ret.push(cost);
            // return cost;
        }
        if memo.get(&((r, c), dir)).is_some_and(|&v| cost < v) {
            continue;
        }

        // if seen.contains(&(r, c)) {
        //     continue;
        // }
        // seen.insert((r, c));

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
                // heap.push((cost + 1, (rr, cc), (dr, dc)));
                queue.push_back((cost + 1, (rr, cc), (dr, dc)));
            }
        }
    }

    println!("ret len: {:?}", ret.len());
    //
    // ret.iter().sorted().for_each(|v| {
    //     println!("{}", v);
    // });

    ret.into_iter().max().unwrap()
}

#[derive(Debug, Clone)]
struct Node {
    pos: (usize, usize),
    edges: HashMap<(usize, usize), usize>,
}

impl Node {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            edges: HashMap::new(),
        }
    }
}

fn grid_to_graph(
    grid: &[Vec<char>],
    end: (usize, usize),
) -> (
    HashMap<(usize, usize), Node>,
    HashSet<(usize, usize)>,
    HashMap<((usize, usize), (usize, usize)), usize>,
) {
    let start = (0, 1);
    let start_dir = (1, 0);
    let start_next = (1, 1);

    // let mut gaph = HashMap::new();
    // graph.insert(start, Node::new(start));

    let mut all_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut all_edges: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((start_next, start_dir, start, 1));

    // let mut seen = HashSet::new();

    while let Some(((r, c), dir, prev_node, weight)) = queue.pop_front() {
        let key = {
            if (r, c).cmp(&prev_node) == std::cmp::Ordering::Less {
                ((r, c), prev_node)
            } else {
                (prev_node, (r, c))
            }
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
        if (r, c) == (13, 5) {
            println!("at (13,5): {:?}", paths);
        }

        if paths.len() == 1 {
            let dir = paths[0];
            let rr = (r as isize + dir.0) as usize;
            let cc = (c as isize + dir.1) as usize;
            queue.push_back(((rr, cc), dir, prev_node, weight + 1));
        } else if paths.len() > 1 || (paths.is_empty() && (r, c) == end) {
            // dbg!((r, c), prev_node);

            // junction
            all_nodes.insert((r, c));
            all_nodes.insert(prev_node);
            all_edges.insert(key, weight);

            // dbg!((r, c), prev_node);
            // println!("queue: {:?}", queue);
            for (dr, dc) in paths {
                let rr = (r as isize + dr) as usize;
                let cc = (c as isize + dc) as usize;
                queue.push_back(((rr, cc), (dr, dc), (r, c), 1));
            }
            // println!("queue: {:?}", queue);
        }
    }
    // assert!(graph.contains_key(&end));
    //
    // graph.iter().for_each(|(_, v)| {
    //     println!("{:?}", v);
    // });
    //
    // graph

    // all_edges.iter().for_each(|(k, v)| {
    //     println!("{:?}: {:?}", k, v);
    // });

    let mut tree = HashMap::new();

    let mut heap = BinaryHeap::new();
    heap.push((0, start));
    let mut memo = HashMap::new();
    let mut edges_seen = HashSet::new();

    while let Some((_cost, curr)) = heap.pop() {
        tree.entry(curr).or_insert(Node::new(curr));

        let edges = all_edges
            .iter()
            .filter(|(&(l, r), _v)| curr == l || curr == r);
        for (&key, &weight) in edges {
            if edges_seen.contains(&key) {
                continue;
            }
            edges_seen.insert(key);
            let next = if key.0 == curr { key.1 } else { key.0 };
            if weight as isize > *memo.get(&next).unwrap_or(&isize::MIN) {
                heap.push((weight, next));
                memo.insert(next, weight as isize);
                // dbg!("before", &curr, &next);
                // graph.iter().for_each(|(_, v)| {
                //     println!("{:?}", v);
                // });
                tree.get_mut(&curr).unwrap().edges.insert(next, weight);

                // dbg!(&curr, &next);
                // graph.iter().for_each(|(_, v)| {
                //     println!("{:?}", v);
                // });
            }
        }
    }
    // dbg!("ready");
    // tree.iter().for_each(|(_, v)| {
    //     println!("{:?}", v);
    // });

    // assert!(graph
    //     .values()
    //     .find(|&n| n.edges.contains_key(&end))
    //     .is_some());

    (tree, all_nodes, all_edges)
}

fn part2(input: &str) -> usize {
    let grid = to_grid(input);

    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let (_, nodes, edges) = grid_to_graph(&grid, end);
    println!("edges len: {}", edges.len());
    let nodes = nodes
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect::<HashMap<_, _>>();

    let start_edges = edges
        .iter()
        .filter(|(&(l, r), _v)| start == l || start == r);

    #[derive(Debug, Clone, PartialEq, Eq)]
    // struct Item(usize, (usize, usize), BTreeSet<(usize, usize)>);
    struct Item(usize, (usize, usize), usize);
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.cmp(&other.0)
        }
    }
    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.0.cmp(&other.0))
        }
    }

    let mut heap = BinaryHeap::new();
    for (&key, &weight) in start_edges {
        println!("start: {:?}", key);
        let mask = 1 << nodes.get(&start).unwrap();
        let next = if key.0 == start { key.1 } else { key.0 };
        heap.push(Item(weight, next, mask));
    }
    let mut memo = HashMap::new();

    let mut ret = 0;

    println!("searching...");

    while let Some(Item(cost, curr, nodes_seen)) = heap.pop() {
        if nodes.contains_key(&curr) && nodes_seen & (1 << *nodes.get(&curr).unwrap()) != 0 {
            continue;
        }

        if curr == end {
            if cost > ret {
                println!("found: {}", cost);
            }
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
                heap.push(Item(cost + weight, next, nodes_seen_next));
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
    assert!(part2(input) > 5850);
    assert_eq!(part2(input), 6318);
}
