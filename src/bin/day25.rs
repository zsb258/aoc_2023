use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("../../inputs/day25.txt");
    println!("Part1: {}", day25(input));
}

fn parse(input: &str) -> (HashMap<String, usize>, Vec<(String, String)>) {
    let mut nodes = HashSet::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let (node, adjs_str) = line.split_once(": ").unwrap();
        nodes.insert(node.to_string());

        for adj in adjs_str.split(' ').map(|s| s.to_string()) {
            nodes.insert(adj.clone());
            edges.push((node.to_string(), adj));
        }
    }

    (
        nodes.into_iter().enumerate().map(|(i, x)| (x, i)).collect(),
        edges,
    )
}

fn day25(input: &str) -> usize {
    let (nodes, edges) = parse(input);

    // https://docs.rs/crate/network-flow/0.1.0
    // https://docs.rs/network-flow/0.1.0/network_flow/graph/struct.Graph.html
    use network_flow::graph::Graph;

    for pair_vec in nodes.iter().combinations(2) {
        let mut flow_graph = Graph::<usize, i64>::new();

        for node in nodes.values() {
            flow_graph.add_node(node);
        }

        let weight = edges.len() as i64;
        for edge in edges.iter() {
            flow_graph.add_edge(nodes[&edge.0], nodes[&edge.1], &weight);
            flow_graph.add_edge(nodes[&edge.1], nodes[&edge.0], &weight);
        }

        // requires to run get_max_flow before get_cut
        let _max_flow = flow_graph.get_max_flow(*pair_vec[0].1, *pair_vec[1].1);
        let min_cut = flow_graph.get_cut(*pair_vec[0].1);
        if min_cut.len() != 1 && min_cut.len() != nodes.len() - 1 {
            return min_cut.len() * (nodes.len() - min_cut.len());
        }
    }

    unreachable!();
}

#[test]
fn example() {
    let example: &str = r"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    assert_eq!(day25(example), 54);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day25.txt");
    assert_eq!(day25(input), 514794);
}
