use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day19.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
use itertools::Itertools;
use std::str::FromStr;

type BluePrint = [[i32; 4]; 4];

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    inventory: [i32; 4],
    robots: [i32; 4],
    minutes_remaining: i32,
}

impl State {
    fn new(minutes_remaining: i32) -> State {
        State {
            inventory: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
            minutes_remaining,
        }
    }
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<BluePrint> {
    input.map(|line| parse_line(line)).collect_vec()
}

fn parse_line(line: String) -> BluePrint {
    let parts = line
        .split(" ")
        .filter_map(|v| i32::from_str(v).ok())
        .collect_vec();
    [
        [parts[0], 0, 0, 0],
        [parts[1], 0, 0, 0],
        [parts[2], parts[3], 0, 0],
        [parts[4], 0, parts[5], 0],
    ]
}

fn evaluate_blueprint(blue_print: &BluePrint, state: State, seen: &mut HashMap<State, i32>) -> i32 {
    // if state.minutes_remaining == 0 {
    //     return state.inventory[GEODE];
    // }
    //
    // let mut score = None;
    // for (i, costs) in blue_print.iter().enumerate().rev() {
    //     if state
    //         .inventory
    //         .iter()
    //         .zip(costs.iter())
    //         .all(|(&i, &c)| i >= c)
    //     {
    //         let mut new_robots = state.robots.clone();
    //         new_robots[i] += 1;
    //         let new_state = State {
    //             inventory: [
    //                 state.inventory[0] + state.robots[0] - costs[0],
    //                 state.inventory[1] + state.robots[1] - costs[1],
    //                 state.inventory[2] + state.robots[2] - costs[2],
    //                 state.inventory[3] + state.robots[3] - costs[3],
    //             ],
    //             robots: new_robots,
    //             minutes_remaining: state.minutes_remaining - 1,
    //         };
    //         score = Some(evaluate_blueprint(blue_print, new_state, seen));
    //         break;
    //     }
    // }
    //
    // if let Some(score) = score {
    //     score
    // } else {
    //     evaluate_blueprint(
    //         blue_print,
    //         State {
    //             inventory: [
    //                 state.inventory[0] + state.robots[0],
    //                 state.inventory[1] + state.robots[1],
    //                 state.inventory[2] + state.robots[2],
    //                 state.inventory[3] + state.robots[3],
    //             ],
    //             robots: state.robots.clone(),
    //             minutes_remaining: state.minutes_remaining - 1,
    //         },
    //         seen,
    //     )
    // }

    if let Some(&score) = seen.get(&state) {
        return score;
    }

    if state.minutes_remaining == 0 {
        return state.inventory[GEODE];
    }

    let mut score = evaluate_blueprint(
        blue_print,
        State {
            inventory: [
                state.inventory[0] + state.robots[0],
                state.inventory[1] + state.robots[1],
                state.inventory[2] + state.robots[2],
                state.inventory[3] + state.robots[3],
            ],
            robots: state.robots.clone(),
            minutes_remaining: state.minutes_remaining - 1,
        },
        seen,
    );

    for (i, costs) in blue_print.iter().enumerate() {
        if state
            .inventory
            .iter()
            .zip(costs.iter())
            .all(|(&i, &c)| i >= c)
        {
            let mut new_robots = state.robots.clone();
            new_robots[i] += 1;
            let new_score = evaluate_blueprint(
                blue_print,
                State {
                    inventory: [
                        state.inventory[0] + state.robots[0] - costs[0],
                        state.inventory[1] + state.robots[1] - costs[1],
                        state.inventory[2] + state.robots[2] - costs[2],
                        state.inventory[3] + state.robots[3] - costs[3],
                    ],
                    robots: new_robots,
                    minutes_remaining: state.minutes_remaining - 1,
                },
                seen,
            );
            score = score.max(new_score);
        }
    }

    seen.insert(state.clone(), score);
    score
}

fn part1(input: impl Iterator<Item = String>) -> i32 {
    let blueprints = parse_input(input);
    let mut res = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut seen = HashMap::new();
        let initial_state = State::new(24);
        let score = evaluate_blueprint(&blueprint, initial_state, &mut seen);
        println!("{}: {}", i, score);
        res += (i + 1) as i32 * score;
    }
    res
}

fn part2(input: impl Iterator<Item = String>) -> i32 {
    let blueprints = parse_input(input);
    let mut res = 0;

    for (i, blueprint) in blueprints.iter().take(3).enumerate() {
        let mut seen = HashMap::new();
        let initial_state = State::new(32);
        let score = evaluate_blueprint(&blueprint, initial_state, &mut seen);
        println!("{}: {}", i, score);
        res *= score;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 33);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 62);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
