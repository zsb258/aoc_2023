use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

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
    fn handle_pulse(
        &self,
        src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        match self {
            Self::FlipFlop(f) => f.handle_pulse(src, pulse),
            Self::Conjunction(c) => c.handle_pulse(src, pulse),
            Self::Broadcaster(b) => b.handle_pulse(src, pulse),
        }
    }

    fn get_dests(&self) -> &[String] {
        match self {
            Self::FlipFlop(f) => &f.dests,
            Self::Conjunction(c) => &c.dests,
            Self::Broadcaster(b) => &b.dests,
        }
    }

    fn get_recents(&self) -> Option<&RefCell<HashMap<String, Pulse>>> {
        match self {
            Self::Conjunction(c) => Some(&c.recents),
            _ => None,
        }
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
}

#[derive(Debug, Clone)]
struct Conjunction {
    recents: RefCell<HashMap<String, Pulse>>,
    dests: Vec<String>,
}

impl Conjunction {
    fn new(dests: Vec<String>) -> Self {
        Self {
            recents: RefCell::new(HashMap::new()),
            dests,
        }
    }

    fn add_input(&self, src: &str, pulse: Pulse) {
        self.recents.borrow_mut().insert(src.to_string(), pulse);
    }

    fn handle_pulse(
        &self,
        src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        self.recents.borrow_mut().insert(src.to_string(), pulse);

        let result_pulse = if self.recents.borrow().values().all(|&p| p == Pulse::High) {
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
}

#[derive(Debug, Clone)]
struct Broadcaster {
    dests: Vec<String>,
}

impl Broadcaster {
    fn new(dests: Vec<String>) -> Self {
        Self { dests }
    }

    fn handle_pulse(
        &self,
        _src: &str,
        pulse: Pulse,
    ) -> Box<dyn Iterator<Item = (&str, Pulse)> + '_> {
        Box::new(self.dests.iter().map(move |dest| (dest.as_str(), pulse)))
    }
}

use Module as M;

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    let mut conjuctions = HashSet::new();

    input.lines().for_each(|line| {
        let (src, dest) = line.split_once(" -> ").unwrap();
        let dest = dest.split(", ").map(|s| s.to_string()).collect();
        let (k, v) = match src.chars().next().unwrap() {
            '%' => (src[1..].to_string(), M::FlipFlop(FlipFlop::new(dest))),
            '&' => {
                conjuctions.insert(&src[1..]);
                (src[1..].to_string(), M::Conjunction(Conjunction::new(dest)))
            }
            _ => (src.to_string(), M::Broadcaster(Broadcaster::new(dest))),
        };
        modules.insert(k, v);
    });

    modules.iter().for_each(|(id, module)| {
        module
            .get_dests()
            .iter()
            .filter(|dest| conjuctions.contains(dest.as_str()))
            .for_each(|dest| {
                if let M::Conjunction(conj) = modules.get(dest).unwrap() {
                    (*conj).add_input(id.as_str(), Pulse::Low);
                }
            });
    });

    modules
}

fn part1(input: &str) -> usize {
    let modules = parse(input);

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
    let modules = parse(input);
    let mut target_input_ids = {
        // assumes only one conjunction feeds into rx
        let (pre_rx, pre_rx_conj) = &modules
            .iter()
            .find(|(_, module)| module.get_dests().iter().any(|dest| dest == "rx"))
            .unwrap();

        pre_rx_conj
            .get_recents()
            .unwrap()
            .borrow()
            .keys()
            .map(|s| ((s.clone(), (*pre_rx).clone()), None))
            .collect::<HashMap<_, _>>()
    };

    let mut ready = Vec::new();
    let mut curr = 0;

    loop {
        let mut queue = VecDeque::new();
        queue.push_back(("", "broadcaster", Pulse::Low));
        curr += 1;
        while let Some((src, dest, pulse)) = queue.pop_front() {
            let key = (src.to_string(), dest.to_string());
            if target_input_ids.contains_key(&key) && pulse == Pulse::High {
                match target_input_ids.get(&key).unwrap() {
                    Some(prev) => {
                        let new = num::integer::lcm(*prev, curr);
                        if new == curr {
                            // cycle found
                            ready.push(*prev);
                            target_input_ids.remove(&key);
                        } else {
                            target_input_ids.insert(key, Some(new));
                        }
                    }
                    None => {
                        target_input_ids.insert(key, Some(curr));
                    }
                }
            }

            if target_input_ids.is_empty() {
                return ready.into_iter().fold(1, num::integer::lcm);
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
