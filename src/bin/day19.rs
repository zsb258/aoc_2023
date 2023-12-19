use std::collections::{HashMap, VecDeque};

fn main() {
    let input: &str = include_str!("../../inputs/day19.txt");
    println!("Part1: {}", part1(input));
    println!("Part2: {}", part2(input));
}

#[derive(Debug)]
enum Cond {
    Lt(usize, usize),
    Gt(usize, usize),
    Nil,
}

impl Cond {
    fn parse(input: &str) -> Self {
        fn custom_convert(s: &str) -> usize {
            match s.chars().next().unwrap() {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!(),
            }
        }
        if let Some((k, v)) = input.split_once('<') {
            Cond::Lt(custom_convert(k), v.parse::<usize>().unwrap())
        } else if let Some((k, v)) = input.split_once('>') {
            Cond::Gt(custom_convert(k), v.parse::<usize>().unwrap())
        } else {
            Cond::Nil
        }
    }
}

#[derive(Debug)]
enum Flow {
    Goto(String),
    Accept,
    Reject,
}

impl Flow {
    fn parse(input: &str) -> Self {
        match input {
            "A" => Flow::Accept,
            "R" => Flow::Reject,
            _ => Flow::Goto(input.to_string()),
        }
    }
}

type WorkflowMap = HashMap<String, Vec<(Cond, Flow)>>;
type Part = Vec<usize>;

fn parse(input: &str) -> (WorkflowMap, Vec<Part>) {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(|l| {
            let (id, rules_str) = l.split_once('{').unwrap();
            let id = id.to_string();
            let rules = rules_str
                .trim_end_matches('}')
                .split(',')
                .map(|r| match r.split_once(':') {
                    Some((k, v)) => (Cond::parse(k), Flow::parse(v)),
                    None => (Cond::Nil, Flow::parse(r)),
                })
                .collect::<Vec<_>>();
            (id, rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts_str
        .lines()
        .map(|l| {
            l.trim_start_matches('{')
                .trim_end_matches('}')
                .splitn(4, ',')
                .map(|s| s.split_once('=').unwrap().1.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);

    parts
        .iter()
        .filter(|p| {
            let mut curr_rules = workflows.get("in").unwrap();
            loop {
                for (cond, flow) in curr_rules {
                    match cond {
                        Cond::Lt(i, v) if p[*i] < *v => match flow {
                            Flow::Goto(k) => {
                                curr_rules = workflows.get(k).unwrap();
                                break;
                            }
                            Flow::Accept => return true,
                            Flow::Reject => return false,
                        },
                        Cond::Gt(i, v) if p[*i] > *v => match flow {
                            Flow::Goto(k) => {
                                curr_rules = workflows.get(k).unwrap();
                                break;
                            }
                            Flow::Accept => return true,
                            Flow::Reject => return false,
                        },
                        Cond::Nil => match flow {
                            Flow::Goto(k) => {
                                curr_rules = workflows.get(k).unwrap();
                                break;
                            }
                            Flow::Accept => return true,
                            Flow::Reject => return false,
                        },
                        _ => continue,
                    }
                }
            }
        })
        .map(|p| p.iter().sum::<usize>())
        .sum()
}

fn part2(input: &str) -> usize {
    let val_range = (1, 4000);
    let (workflows, _parts) = parse(input);

    let mut accepted = Vec::new();

    let mut queue = VecDeque::new();
    queue.push_back((workflows.get("in").unwrap(), vec![val_range; 4]));

    while let Some((rules, mut rgs)) = queue.pop_front() {
        dbg!(&rgs);
        for (cond, flow) in rules {
            let mut tmp = rgs.clone();
            match cond {
                Cond::Lt(i, v) => {
                    tmp[*i].1 = std::cmp::min(tmp[*i].1, *v - 1);
                    rgs[*i].0 = std::cmp::max(rgs[*i].0, *v);
                }

                Cond::Gt(i, v) => {
                    tmp[*i].0 = std::cmp::max(tmp[*i].0, *v + 1);
                    rgs[*i].1 = std::cmp::min(rgs[*i].1, *v);
                }
                Cond::Nil => (),
            };
            dbg!(&cond);
            dbg!(&tmp);
            dbg!(&flow);
            match flow {
                Flow::Goto(k) => {
                    queue.push_back((workflows.get(k).unwrap(), tmp.clone()));
                }
                Flow::Accept => accepted.push(tmp),
                Flow::Reject => (),
            };
        }
    }

    dbg!("accepted");

    let mut sum = 0;
    for rg in accepted.iter() {
        for (l, u) in rg.iter() {
            print!("({:>4}, {:>4})  ", l, u);
        }
        println!();

        sum += rg
            .iter()
            .fold(1, |acc, (lower, upper)| acc * (upper - lower + 1));
    }

    sum
}

#[test]
fn example() {
    let example: &str = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(part1(example), 19114);
    assert_eq!(part2(example), 167409079868000);
}

#[test]
fn answer() {
    let input: &str = include_str!("../../inputs/day19.txt");
    assert_eq!(part1(input), 352052);
    assert_eq!(part2(input), 116606738659695);
}
