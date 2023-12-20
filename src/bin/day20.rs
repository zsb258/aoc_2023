use std::collections::{HashMap, HashSet, VecDeque};

use num;

fn main() {
    let input: &str = include_str!("../../inputs/day20.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

impl Module {
    fn handle_pulse(&mut self, src: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        match self {
            Self::FlipFlop(f) => f.handle_pulse(src, pulse),
            Self::Conjunction(c) => c.handle_pulse(src, pulse),
            Self::Broadcaster(b) => b.handle_pulse(src, pulse),
        }
    }

    fn get_dests(&self) -> Vec<String> {
        match self {
            Self::FlipFlop(f) => f.dests.clone(),
            Self::Conjunction(c) => c.dests.clone(),
            Self::Broadcaster(b) => b.dests.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    on: bool,
    dests: Vec<String>,
}

impl FlipFlop {
    fn new_boxed(dests: Vec<String>) -> Self {
        Self { on: false, dests }
    }

    fn handle_pulse(&mut self, _src: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        let mut ret = Vec::new();
        match (pulse, self.on) {
            (Pulse::High, _) => (),
            (Pulse::Low, false) => {
                self.on = true;
                for dest in &self.dests {
                    ret.push((dest.clone(), Pulse::High));
                }
            }
            (Pulse::Low, true) => {
                self.on = false;
                for dest in &self.dests {
                    ret.push((dest.clone(), Pulse::Low));
                }
            }
        }

        ret
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    recents: HashMap<String, Pulse>,
    dests: Vec<String>,
}

impl Conjunction {
    fn new_boxed(dests: Vec<String>) -> Self {
        Self {
            recents: HashMap::new(),
            dests,
        }
    }

    fn add_input(&mut self, src: String, pulse: Pulse) {
        self.recents.insert(src, pulse);
    }

    fn handle_pulse(&mut self, src: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.recents.insert(src, pulse);
        let all_high = self.recents.values().all(|&p| p == Pulse::High);

        let mut ret = Vec::new();
        if all_high {
            for dest in &self.dests {
                ret.push((dest.clone(), Pulse::Low));
            }
        } else {
            for dest in &self.dests {
                ret.push((dest.clone(), Pulse::High));
            }
        }

        ret
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    dests: Vec<String>,
}

impl Broadcaster {
    fn new_boxed(dests: Vec<String>) -> Self {
        Self { dests }
    }

    fn handle_pulse(&mut self, _src: String, pulse: Pulse) -> Vec<(String, Pulse)> {
        let mut ret = Vec::new();
        for dest in &self.dests {
            ret.push((dest.clone(), pulse));
        }

        ret
    }
}

use Module as M;

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut conjuctions: HashSet<String> = HashSet::new();
    input.lines().for_each(|line| {
        let (src, dest) = line.split_once(" -> ").unwrap();
        let dest = dest.split(", ").map(|s| s.to_string()).collect();
        match src.chars().next().unwrap() {
            '%' => {
                modules.insert(src[1..].to_string(), M::FlipFlop(FlipFlop::new_boxed(dest)));
            }
            '&' => {
                let id = src[1..].to_string();
                modules.insert(id.clone(), M::Conjunction(Conjunction::new_boxed(dest)));
                conjuctions.insert(id);
            }
            _ => {
                modules.insert(
                    src.to_string(),
                    M::Broadcaster(Broadcaster::new_boxed(dest)),
                );
            }
        }
    });

    for (id, module) in modules.clone().iter() {
        for dest in module.get_dests().iter() {
            if conjuctions.contains(dest) {
                if let M::Conjunction(conj) = modules.get_mut(dest).unwrap() {
                    (*conj).add_input(id.clone(), Pulse::Low);
                }
            }
        }
    }

    modules
}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);
    // dbg!(&modules);
    let mut lcount = 0;
    let mut hcount = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        queue.push_back(("".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((src, dest, pulse)) = queue.pop_front() {
            // dbg!("handling", &src, &pulse, &dest);
            match &pulse {
                Pulse::Low => lcount += 1,
                Pulse::High => hcount += 1,
            }

            if let Some(module) = modules.get_mut(&dest) {
                for (next_dest, next_pulse) in module.handle_pulse(src, pulse) {
                    // dbg!(&next_pulse, &next_dest);
                    queue.push_back((dest.clone(), next_dest, next_pulse));
                }
                // dbg!("handle end");
            }
            // dbg!("queue", &queue);
        }
        // dbg!(lcount, hcount);
    }

    lcount * hcount
}

fn part2(input: &str) -> usize {
    let mut modules = parse(input);
    // dbg!(&modules);
    let target_input_ids: Vec<(String, String)> = {
        if let M::Conjunction(conj) = modules.clone().get("ql").unwrap() {
            conj.recents
                .keys()
                .clone()
                .map(|s| ("ql".to_string(), (*s).clone()))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    };
    dbg!(&target_input_ids);

    target_input_ids
        .into_iter()
        .flat_map(|(dest, input)| step(modules.clone(), dest, input))
        .fold(1, num::integer::lcm)
}

fn step(mut modules: HashMap<String, Module>, dest_id: String, input_id: String) -> Vec<usize> {
    dbg!(&dest_id, &input_id);
    let mut steps = Vec::new();

    let mut pressed = 0;

    loop {
        let mut queue = VecDeque::new();
        queue.push_back(("".to_string(), "broadcaster".to_string(), Pulse::Low));
        pressed += 1;
        while let Some((src, dest, pulse)) = queue.pop_front() {
            if src == input_id && dest == dest_id && pulse == Pulse::High {
                dbg!(&dest_id, &input_id, &pressed);
                println!("found {}", &pressed);
                if steps.is_empty() {
                    steps.push(pressed);
                } else {
                    if steps
                        .iter()
                        .fold(pressed, |acc, curr| num::integer::lcm(acc, *curr))
                        == pressed
                    {
                        return steps;
                    } else {
                        steps.push(pressed);
                    }
                }
            }

            if let Some(module) = modules.get_mut(&dest) {
                for (next_dest, next_pulse) in module.handle_pulse(src, pulse) {
                    // dbg!(&next_pulse, &next_dest);
                    queue.push_back((dest.clone(), next_dest, next_pulse));
                }
                // dbg!("handle end");
            }
            // dbg!("queue", &queue);
        }
    }
}

#[test]
fn example() {
    let example: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part1(example), 32000000);
    // assert_eq!(part2(example), 167409079868000);

    let example: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(example), 11687500);
}

// #[test]
// fn answer() {
//     let input: &str = include_str!("../../inputs/day20.txt");
//     assert_eq!(part1(input), 352052);
//     // assert_eq!(part2(input), 116606738659695);
// }
