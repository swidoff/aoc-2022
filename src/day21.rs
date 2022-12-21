use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day21.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

enum Job {
    Number(i64),
    Formula(String, String, String),
}

fn parse_input(input: impl Iterator<Item = String>) -> HashMap<String, Job> {
    let mut res = HashMap::new();
    for line in input {
        let (monkey, job_str) = line.split(": ").collect_tuple().unwrap();
        let words = job_str.split_whitespace().collect_vec();
        let job = match words.as_slice() {
            [m1, op, m2] => Job::Formula(op.to_string(), m1.to_string(), m2.to_string()),
            [n] => Job::Number(i64::from_str(n).unwrap()),
            _ => panic!(),
        };
        res.insert(monkey.to_string(), job);
    }
    res
}

fn part1(input: impl Iterator<Item = String>) -> i64 {
    let jobs = parse_input(input);
    solve_part1(&jobs, &"root".to_string())
}

fn solve_part1(jobs: &HashMap<String, Job>, monkey: &String) -> i64 {
    match jobs.get(monkey).unwrap() {
        Job::Number(v) => *v,
        Job::Formula(op, m1, m2) => {
            let v1 = solve_part1(jobs, m1);
            let v2 = solve_part1(jobs, m2);
            match op.as_str() {
                "+" => v1 + v2,
                "-" => v1 - v2,
                "/" => v1 / v2,
                "*" => v1 * v2,
                _ => panic!(),
            }
        }
    }
}

fn part2(input: impl Iterator<Item = String>) -> i64 {
    let jobs = parse_input(input);
    match jobs.get(&"root".to_string()).unwrap() {
        Job::Formula(_, m1, m2) => {
            if has_human(&jobs, m1) {
                let value = solve_part1(&jobs, m2);
                solve_part2(&jobs, m1, value)
            } else {
                let value = solve_part1(&jobs, m1);
                solve_part2(&jobs, m2, value)
            }
        }
        _ => panic!(),
    }
}

fn solve_part2(jobs: &HashMap<String, Job>, monkey: &String, value: i64) -> i64 {
    if monkey == "humn" {
        value
    } else {
        match jobs.get(monkey).unwrap() {
            Job::Formula(op, m1, m2) => {
                if has_human(&jobs, m1) {
                    let operand = solve_part1(&jobs, m2);
                    let new_value = match op.as_str() {
                        "+" => value - operand,
                        "-" => value + operand,
                        "/" => value * operand,
                        "*" => value / operand,
                        _ => panic!(),
                    };
                    solve_part2(&jobs, m1, new_value)
                } else {
                    let operand = solve_part1(&jobs, m1);
                    let new_value = match op.as_str() {
                        "+" => value - operand,
                        "-" => -(value - operand),
                        "/" => operand / value,
                        "*" => value / operand,
                        _ => panic!(),
                    };
                    solve_part2(&jobs, m2, new_value)
                }
            }
            _ => panic!(),
        }
    }
}

fn has_human(jobs: &HashMap<String, Job>, monkey: &String) -> bool {
    if monkey == "humn" {
        true
    } else {
        match jobs.get(monkey).unwrap() {
            Job::Number(_) => false,
            Job::Formula(_, m1, m2) => has_human(jobs, m1) || has_human(jobs, m2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 152);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 301);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
