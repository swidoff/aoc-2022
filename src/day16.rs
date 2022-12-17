use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

struct Tunnel {
    target: String,
    steps: i64,
}

struct Valve {
    bit: u64,
    flow: i64,
    tunnels: Vec<Tunnel>,
}

fn parse_input(input: impl Iterator<Item = String>) -> HashMap<String, Valve> {
    let mut res = HashMap::new();
    for (i, line) in input.enumerate() {
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap().to_string();
        let flow = i64::from_str(parts.next().unwrap()).unwrap();
        let tunnels = parts
            .map(|s| Tunnel {
                target: s.to_string(),
                steps: 1,
            })
            .collect_vec();

        let bit = 1 << i;
        res.insert(name, Valve { bit, flow, tunnels });
    }
    res
}

struct CollapseState {
    loc: String,
    steps: i64,
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
                    if *distances.get(target).unwrap_or(&i64::MAX) > new_steps {
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
                    bit: valve.bit,
                    flow: valve.flow,
                    tunnels,
                },
            );
        }
    }
    new_system
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct StatePart1 {
    loc: String,
    opened: u64,
    minutes: i64,
}

fn solve_part1(system: &HashMap<String, Valve>, state: StatePart1) -> i64 {
    let mut best_score = 0;
    let valve = system.get(&state.loc).unwrap();
    for Tunnel { target, steps } in &valve.tunnels {
        let target_valve = system.get(target).unwrap();
        let new_minutes = state.minutes + steps + 1;
        if new_minutes <= 30 && state.opened & target_valve.bit == 0 {
            let new_state = StatePart1 {
                loc: target.clone(),
                opened: state.opened | target_valve.bit,
                minutes: new_minutes,
            };
            let score = solve_part1(&system, new_state);
            best_score = best_score.max(score);
        }
    }
    valve.flow * (30 - state.minutes) + best_score
}

fn part1(input: impl Iterator<Item = String>) -> i64 {
    let system = collapse_system(parse_input(input));
    let initial_state = StatePart1 {
        loc: "AA".to_string(),
        opened: 0,
        minutes: 0,
    };
    solve_part1(&system, initial_state)
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct StatePart2 {
    locs: [String; 2],
    remaining_minutes: [i64; 2],
    opened: u64,
}

fn solve_part2(
    system: &HashMap<String, Valve>,
    state: StatePart2,
    best_scores: &mut HashMap<StatePart2, i64>,
) -> i64 {
    if let Some(&score) = best_scores.get(&state) {
        return score;
    }

    let turn = if state.remaining_minutes[0] >= state.remaining_minutes[1] {
        0
    } else {
        1
    };

    let mut best_score = 0;
    let valves = [
        system.get(&state.locs[0]).unwrap(),
        system.get(&state.locs[1]).unwrap(),
    ];
    for Tunnel { target, steps } in &valves[turn].tunnels {
        let target_valve = system.get(target).unwrap();
        let new_remaining_minutes = state.remaining_minutes[turn] - steps - 1;
        if new_remaining_minutes >= 0 && state.opened & target_valve.bit == 0 {
            let mut remaining_minutes = state.remaining_minutes.clone();
            remaining_minutes[turn] = new_remaining_minutes;

            let mut locs = state.locs.clone();
            locs[turn] = target.clone();

            let new_state = StatePart2 {
                remaining_minutes,
                locs,
                opened: state.opened | target_valve.bit,
            };
            let score = solve_part2(&system, new_state, best_scores);
            best_score = best_score.max(score);
        }
    }

    let score = valves[turn].flow * state.remaining_minutes[turn] + best_score;
    best_scores.insert(
        StatePart2 {
            locs: [state.locs[1].clone(), state.locs[0].clone()],
            remaining_minutes: [
                state.remaining_minutes[1].clone(),
                state.remaining_minutes[0].clone(),
            ],
            opened: state.opened.clone(),
        },
        score,
    );
    best_scores.insert(state, score);
    score
}

fn part2(input: impl Iterator<Item = String>) -> i64 {
    let system = collapse_system(parse_input(input));
    let initial_state = StatePart2 {
        locs: ["AA".to_string(), "AA".to_string()],
        remaining_minutes: [26, 26],
        opened: 0,
    };
    let mut best_scores = HashMap::new();
    solve_part2(&system, initial_state, &mut best_scores)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

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

    const INPUT: &str = "GJ 14 UV AO MM UD GM
HE 0 QE SV
ET 0 LU SB
SG 0 FF SB
LC 0 QJ GM
EE 13 RE BR
AA 0 QC ZR NT JG FO
TF 0 LU MM
GO 0 LB AH
QE 24 LG HE
MI 0 KU FF
BR 0 HY EE
UV 0 GP GJ
EH 0 UU FF
WK 0 HY EL
NT 0 FF AA
KI 0 OQ AO
AH 22 GO RE
EL 0 WK SQ
GP 0 SB UV
GM 0 LC GJ
LU 9 UU DW TF ET ML
LB 0 GO VI
QC 0 ML AA
JJ 0 QJ DV
MM 0 TF GJ
VI 18 LB
NV 0 SB KU
VT 0 HY JG
RE 0 AH EE
FO 0 SB AA
DV 10 JH UD JJ
SQ 12 EL QA
OQ 23 KI IV JS
FF 3 EU NT SG MI EH
IV 0 LG OQ
HY 8 VT BR WK
ML 0 LU QC
JS 0 EM OQ
KU 5 MI VL NV HU DW
QA 0 OS SQ
EU 0 FF OS
SV 0 QJ HE
JG 0 AA VT
DW 0 LU KU
UD 0 DV GJ
QJ 17 JJ SV LC EM YA
HU 0 JH KU
ZR 0 AA VL
YA 0 QJ OS
JH 0 HU DV
OS 15 EU YA QA
LG 0 QE IV
SB 4 FO SG NV GP ET
UU 0 EH LU
VL 0 ZR KU
AO 0 GJ KI
EM 0 QJ JS
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 1651);
    }

    #[test]
    fn test_part1() {
        let res = part1(INPUT.lines().map(|v| v.to_string()));
        println!("{}", res);
        assert_eq!(res, 1728);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 1707);
    }

    #[test]
    fn test_part2() {
        let res = part2(INPUT.lines().map(|v| v.to_string()));
        println!("{}", res);
        assert_eq!(res, 2304);
    }
}
