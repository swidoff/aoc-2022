use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day19.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

// const ORE: usize = 0;
// const CLAY: usize = 1;
// const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
use itertools::Itertools;
use std::str::FromStr;

type Costs = [i32; 4];
type BluePrint = [Costs; 4];

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    inventory: [i32; 4],
    robots: [i32; 4],
    minutes: i32,
}

impl State {
    fn new() -> State {
        State {
            inventory: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
            minutes: 0,
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

fn evaluate_blueprint(
    blue_print: &BluePrint,
    state: State,
    max_minutes: i32,
    seen: &mut HashMap<State, i32>,
) -> i32 {
    if let Some(&score) = seen.get(&state) {
        return score;
    }

    if state.minutes == max_minutes {
        return state.inventory[GEODE];
    }

    let mut bought = false;
    let mut score = 0;
    let remaining_minutes = max_minutes - state.minutes;
    for (i, costs) in blue_print.iter().enumerate().rev() {
        if let Some(n) = turns_before_purchase(&state, costs) {
            if n < max_minutes - state.minutes {
                let mut new_robots = state.robots.clone();
                new_robots[i] += 1;
                let new_score = evaluate_blueprint(
                    blue_print,
                    State {
                        inventory: [
                            state.inventory[0] + state.robots[0] * n - costs[0],
                            state.inventory[1] + state.robots[1] * n - costs[1],
                            state.inventory[2] + state.robots[2] * n - costs[2],
                            state.inventory[3] + state.robots[3] * n - costs[3],
                        ],
                        robots: new_robots,
                        minutes: state.minutes + n,
                    },
                    max_minutes,
                    seen,
                );
                score = score.max(new_score);
                bought = true;
                if i == GEODE && n == 1 {
                    break;
                }
                // else if i == CLAY && state.robots[i] > 0 {
                //     break;
                // }
            }
        }
    }

    if !bought {
        score = evaluate_blueprint(
            blue_print,
            State {
                inventory: [
                    state.inventory[0] + state.robots[0] * remaining_minutes,
                    state.inventory[1] + state.robots[1] * remaining_minutes,
                    state.inventory[2] + state.robots[2] * remaining_minutes,
                    state.inventory[3] + state.robots[3] * remaining_minutes,
                ],
                robots: state.robots.clone(),
                minutes: max_minutes,
            },
            max_minutes,
            seen,
        );
    }

    seen.insert(state.clone(), score);
    score
}

fn turns_before_purchase(state: &State, costs: &Costs) -> Option<i32> {
    let inventory = state.inventory;
    let robots = state.robots;
    let mut turns = 0;
    if (0..4).all(|i| costs[i] == 0 || (robots[i] > 0 || inventory[i] >= costs[i])) {
        for i in 0..4 {
            if costs[i] > 0 {
                if robots[i] > 0 {
                    let mut cost_turns = (costs[i] - inventory[i]) / robots[i];
                    if (costs[i] - inventory[i]) % robots[i] > 0 {
                        cost_turns += 1;
                    }

                    turns = turns.max(cost_turns)
                } else {
                    turns = turns.max(0)
                }
            }
        }
        Some(turns + 1)
    } else {
        None
    }
}

fn part1(input: impl Iterator<Item = String>) -> i32 {
    let blueprints = parse_input(input);
    let mut res = 0;

    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut seen = HashMap::new();
        let initial_state = State::new();
        let score = evaluate_blueprint(&blueprint, initial_state, 24, &mut seen);
        println!("{}: {}", i, score);
        res += (i + 1) as i32 * score;
    }
    res
}

fn part2(input: impl Iterator<Item = String>) -> i32 {
    let blueprints = parse_input(input);
    let mut res = 1;

    for (i, blueprint) in blueprints.iter().take(3).enumerate() {
        let mut seen = HashMap::new();
        let initial_state = State::new();
        let score = evaluate_blueprint(&blueprint, initial_state, 32, &mut seen);
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
        assert_eq!(res, 1023);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 62);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // Too low: 13260
        assert_eq!(res, 13520);
    }
}
