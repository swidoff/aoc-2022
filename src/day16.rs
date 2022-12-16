use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

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

struct StatePart1 {
    score: u64,
    minute: u64,
    loc: String,
    opened: HashMap<String, u64>,
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let system = collapse_system(parse_input(input));
    let mut q = VecDeque::new();
    let mut solutions = Vec::new();
    q.push_back(StatePart1 {
        score: 0,
        minute: 0,
        loc: "AA".to_string(),
        opened: Default::default(),
    });
    let mut final_score = 0;

    while let Some(StatePart1 {
        score,
        minute,
        loc,
        opened,
    }) = q.pop_back()
    {
        final_score = final_score.max(score);

        for Tunnel { target, steps } in &system.get(&loc).unwrap().tunnels {
            if !opened.contains_key(target) {
                let mut opened = opened.clone();
                let new_minute = minute + steps + 1;
                if new_minute > 30 {
                    solutions.push((score, opened));
                    continue;
                }

                opened.insert(target.clone(), new_minute);
                let new_score = score
                    + if new_minute > 29 {
                        0
                    } else {
                        (30 - new_minute) * system.get(target).unwrap().flow
                    };

                q.push_back(StatePart1 {
                    score: new_score,
                    minute: new_minute,
                    loc: target.clone(),
                    opened,
                })
            }
        }
    }
    println!("{}", solutions.len());
    final_score
}

struct StatePart2 {
    score: u64,
    pos: [(String, u64); 2],
    opened: HashMap<String, u64>,
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let system = collapse_system(parse_input(input));
    let mut q = VecDeque::new();
    q.push_back(StatePart2 {
        score: 0,
        pos: [("AA".to_string(), 0), ("AA".to_string(), 0)],
        opened: Default::default(),
    });
    let mut final_score = 0;
    let mut seen = HashMap::new();

    while let Some(StatePart2 { score, pos, opened }) = q.pop_back() {
        if score > final_score {
            final_score = score;
        }

        if let Some(old_score) = seen.get(&pos) {
            if score < *old_score {
                continue;
            }
        } else {
            let new_pos = [pos[0].clone(), pos[1].clone()];
            seen.insert(new_pos.clone(), score);
            seen.insert(pos.clone(), score);
        }

        let turn = if pos[0].1 <= pos[1].1 { 0 } else { 1 };
        let (loc, minute) = &pos[turn];

        for Tunnel { target, steps } in &system.get(loc).unwrap().tunnels {
            if !opened.contains_key(target) {
                let mut opened = opened.clone();
                let new_minute = *minute + steps + 1;
                if new_minute > 26 {
                    continue;
                }

                opened.insert(target.clone(), new_minute);
                let new_score = score
                    + if new_minute > 25 {
                        0
                    } else {
                        (26 - new_minute) * system.get(target).unwrap().flow
                    };

                let mut new_pos = pos.clone();
                new_pos[turn] = (target.clone(), new_minute);

                q.push_back(StatePart2 {
                    score: new_score,
                    pos: new_pos,
                    opened,
                })
            }
        }
    }
    final_score
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
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 1707);
    }

    #[test]
    fn test_part2() {
        let res = part2(INPUT.lines().map(|v| v.to_string()));
        println!("{}", res);
        assert_eq!(res, 0);

        // 2168
    }
}
