use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day16.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Tunnel {
    target: String,
    steps: u64,
}

struct Valve {
    flow: u64,
    tunnels: Vec<Tunnel>,
}

fn parse_input(input: impl Iterator<Item = String>) -> HashMap<String, Valve> {
    let mut res = HashMap::new();
    for line in input {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let flow = u64::from_str(parts.next().unwrap()).unwrap();
        let tunnels = parts
            .map(|s| Tunnel {
                target: s.to_string(),
                steps: 1,
            })
            .collect_vec();
        res.insert(name, Valve { flow, tunnels });
    }
    res
}

struct CollapseState {
    loc: String,
    steps: u64,
}

fn collapse_system(system: HashMap<String, Valve>) -> HashMap<String, Valve> {
    let mut new_system = HashMap::new();
    for (valve_name, valve) in system.iter() {
        if valve_name.as_str() == "AA" || valve.flow > 0 {
            let mut q = VecDeque::new();
            let mut distances = HashMap::new();
            distances.insert(valve_name.clone(), 0);
            q.push_back(CollapseState {
                loc: valve_name.clone(),
                steps: 0,
            });
            while let Some(CollapseState { loc, steps }) = q.pop_back() {
                let new_steps = steps + 1;
                for Tunnel { target, .. } in &system.get(&loc).unwrap().tunnels {
                    if *distances.get(target).unwrap_or(&u64::MAX) > new_steps {
                        distances.insert(target.clone(), new_steps);
                        q.push_back(CollapseState {
                            loc: target.clone(),
                            steps: new_steps,
                        })
                    }
                }
            }

            let mut tunnels = Vec::new();
            for (target, steps) in distances {
                if system.get(&target).unwrap().flow > 0 {
                    tunnels.push(Tunnel {
                        target: target.clone(),
                        steps,
                    })
                }
            }
            new_system.insert(
                valve_name.clone(),
                Valve {
                    flow: valve.flow,
                    tunnels,
                },
            );
        }
    }
    new_system
}

#[derive(Clone, Eq, Debug, PartialEq)]
struct SolutionState {
    score: u64,
    minute: u64,
    loc: String,
    opened: HashMap<String, u64>,
}

impl Ord for SolutionState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for SolutionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let system = collapse_system(parse_input(input));
    let mut q = BinaryHeap::new();
    q.push(SolutionState {
        score: 0,
        minute: 1,
        loc: "AA".to_string(),
        opened: Default::default(),
    });

    while let Some(SolutionState {
        score,
        minute,
        loc,
        opened,
    }) = q.pop()
    {
        if minute > 30 {
            continue;
        } else if minute == 30 {
            println!("{:?}", opened);
            return score;
        }

        for Tunnel { target, steps } in &system.get(&loc).unwrap().tunnels {
            if !opened.contains_key(target) && system.get(target).unwrap().flow > 0 {
                let mut opened = opened.clone();
                let new_minute = minute + steps + 1;
                opened.insert(target.clone(), new_minute);
                let new_score = score
                    + if new_minute > 30 {
                        0
                    } else {
                        (30 - new_minute) * system.get(target).unwrap().flow
                    };

                q.push(SolutionState {
                    score: new_score,
                    minute: new_minute,
                    loc: target.clone(),
                    opened,
                })
            }
        }
    }
    0
}

fn part2(_input: impl Iterator<Item = String>) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "AA 0 DD II BB
BB 13 CC AA
CC 2 DD BB
DD 20 CC AA EE
EE 3 FF DD
FF 0 EE GG
GG 0 FF HH
HH 22 GG
II 0 AA JJ
JJ 21 II
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 1651);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 0);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
