use std::cell::RefCell;
use std::collections::hash_map::{Entry, HashMap};
use std::collections::{HashSet, VecDeque};

fn main() {
    let input: &str = include_str!("../../inputs/day20.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Pulse {
    Low,
    High,
}

trait Module {
    fn handle_pulse(&self, src: &str, pulse: Pulse)
        -> Box<dyn Iterator<Item = (&str, Pulse)> + '_>;

    fn get_dests(&self) -> &[String];

    // using interior mutability
    fn add_input(&self, _src: &str, _pulse: Pulse) {}

    fn get_inputs(&self) -> Option<Vec<String>> {
        None
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    on: RefCell<bool>,
    dests: Vec<String>,
}

impl FlipFlop {
    fn new(dests: Vec<String>) -> Self {
        Self {
            on: RefCell::new(false),
            dests,
        }
    }
}

impl Module for FlipFlop {
    fn handle_pulse(
        &self,
        _src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        match pulse {
            Pulse::High => Box::new(std::iter::empty()),
            Pulse::Low => {
                let curr_on = *self.on.borrow();
                *self.on.borrow_mut() = !curr_on;

                let result_pulse = if *self.on.borrow() {
                    Pulse::High
                } else {
                    Pulse::Low
                };
                Box::new(
                    self.dests
                        .iter()
                        .map(move |dest| (dest.as_str(), result_pulse)),
                )
            }
        }
    }

    fn get_dests(&self) -> &[String] {
        &self.dests
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    inputs: RefCell<HashMap<String, Pulse>>,
    dests: Vec<String>,
}

impl Conjunction {
    fn new(dests: Vec<String>) -> Self {
        Self {
            inputs: RefCell::new(HashMap::new()),
            dests,
        }
    }
}

impl Module for Conjunction {
    fn handle_pulse(
        &self,
        src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        self.inputs.borrow_mut().insert(src.to_string(), pulse);

        let result_pulse = if self.inputs.borrow().values().all(|&p| p == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        };

        Box::new(
            self.dests
                .iter()
                .map(move |dest| (dest.as_str(), result_pulse)),
        )
    }

    fn get_dests(&self) -> &[String] {
        &self.dests
    }

    fn add_input(&self, src: &str, pulse: Pulse) {
        self.inputs.borrow_mut().insert(src.to_string(), pulse);
    }

    fn get_inputs(&self) -> Option<Vec<String>> {
        Some(
            self.inputs
                .borrow()
                .clone()
                .keys()
                .map(String::from)
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    dests: Vec<String>,
}

impl Broadcaster {
    fn new(dests: Vec<String>) -> Self {
        Self { dests }
    }
}

impl Module for Broadcaster {
    fn handle_pulse(
        &self,
        _src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        Box::new(self.dests.iter().map(move |dest| (dest.as_str(), pulse)))
    }

    fn get_dests(&self) -> &[String] {
        &self.dests
    }
}

fn parse(input: &str) -> HashMap<String, Box<dyn Module>> {
    let mut modules = HashMap::new();
    let mut conjuctions = HashSet::new();

    input.lines().for_each(|line| {
        let (src, dest) = line.split_once(" -> ").unwrap();
        let dest = dest.split(", ").map(|s| s.to_string()).collect();
        let (k, v) = match src.chars().next().unwrap() {
            '%' => (
                src[1..].to_string(),
                Box::new(FlipFlop::new(dest)) as Box<dyn Module>,
            ),
            '&' => {
                conjuctions.insert(&src[1..]);
                (
                    src[1..].to_string(),
                    Box::new(Conjunction::new(dest)) as Box<dyn Module>,
                )
            }
            _ => (
                src.to_string(),
                Box::new(Broadcaster::new(dest)) as Box<dyn Module>,
            ),
        };
        modules.insert(k, v);
    });

    modules.iter().for_each(|(id, module)| {
        module
            .get_dests()
            .iter()
            .filter(|dest| conjuctions.contains(dest.as_str()))
            .for_each(|dest| {
                modules
                    .get(dest)
                    .unwrap()
                    .add_input(id.as_str(), Pulse::Low);
            });
    });

    modules
}

fn part1(input: &str) -> usize {
    #[allow(unused_mut)] // Module has interior mutability
    let mut modules = parse(input);

    (0..1000)
        .fold([0, 0], |mut acc, _| {
            let mut queue = VecDeque::new();
            queue.push_back(("", "broadcaster", Pulse::Low));

            while let Some((src, dest, pulse)) = queue.pop_front() {
                match &pulse {
                    Pulse::Low => acc[0] += 1,
                    Pulse::High => acc[1] += 1,
                }

                if let Some(module) = modules.get(&dest.to_string()) {
                    for (next_dest, next_pulse) in module.handle_pulse(src, pulse) {
                        queue.push_back((&dest, next_dest, next_pulse));
                    }
                }
            }

            acc
        })
        .iter()
        .product()
}

fn part2(input: &str) -> usize {
    #[allow(unused_mut)] // Module has interior mutability
    let mut modules = parse(input);

    // assumes only one conjunction feeds into rx
    let (pre_rx, pre_rx_conj) = &modules
        .iter()
        .find(|(_, module)| module.get_dests().iter().any(|dest| dest == "rx"))
        .unwrap();

    // appeasing the borrow checker
    let binding = pre_rx_conj.get_inputs().unwrap();
    let mut targets: HashMap<_, Option<usize>> = binding
        .iter()
        .map(|s| ((s.as_str(), (*pre_rx).as_str()), None))
        .collect::<HashMap<_, _>>();

    let mut found = Vec::new();
    let mut curr = 0;

    loop {
        let mut queue = VecDeque::new();
        queue.push_back(("", "broadcaster", Pulse::Low));
        curr += 1;

        while let Some((src, dest, pulse)) = queue.pop_front() {
            if let Entry::Occupied(mut o) = targets.entry((src, dest)) {
                if pulse == Pulse::High {
                    match o.get() {
                        Some(prev) => {
                            let new = num::integer::lcm(*prev, curr);
                            if new == curr {
                                // cycle found
                                found.push(*prev);
                                o.remove();
                            } else {
                                o.insert(Some(new));
                            }
                        }
                        None => {
                            o.insert(Some(curr));
                        }
                    }
                }
            }

            if targets.is_empty() {
                return found.into_iter().fold(1, num::integer::lcm);
            }

            if let Some(module) = modules.get(&dest.to_string()) {
                for (next_dest, next_pulse) in module.handle_pulse(src, pulse) {
                    queue.push_back((dest, next_dest, next_pulse));
                }
            }
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

    let example: &str = r"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(example), 11687500);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day20.txt");
    assert_eq!(part1(input), 787056720);
    assert_eq!(part2(input), 212986464842911);
}
