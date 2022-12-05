use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day5.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Input {
    stacks: Vec<VecDeque<char>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Input {
    fn from_iter(iter: impl Iterator<Item = String>) -> Input {
        let (iter1, iter2) = iter.tee();

        let mut stacks = Vec::new();
        for line in iter1.take_while(|line| !line.is_empty()) {
            for (i, letter) in line.chars().skip(1).step_by(4).enumerate() {
                if (i + 1) > stacks.len() {
                    stacks.push(VecDeque::new())
                }
                if letter.is_alphabetic() {
                    stacks[i].push_front(letter);
                }
            }
        }

        let instructions = iter2
            .skip_while(|line| !line.is_empty())
            .skip(1)
            .map(|line| {
                line.split(' ')
                    .skip(1)
                    .step_by(2)
                    .map(|c| usize::from_str(c).unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();

        Input {
            stacks,
            instructions,
        }
    }
}

fn part1(iter: impl Iterator<Item = String>) -> String {
    let Input {
        mut stacks,
        instructions,
    } = Input::from_iter(iter);

    for &(num, from, to) in instructions.iter() {
        for _i in 0..num {
            let c = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(c);
        }
    }
    tops(&stacks)
}

fn part2(iter: impl Iterator<Item = String>) -> String {
    let Input {
        mut stacks,
        instructions,
    } = Input::from_iter(iter);

    for &(num, from, to) in instructions.iter() {
        let len = stacks[from - 1].len();

        // Push the last num elements from the `from` stack on to the `to` stack.
        for i in 0..num {
            let &c = stacks[from - 1].get(len - num + i).unwrap();
            stacks[to - 1].push_back(c);
        }

        // Now pop those elements off the `from` stack.
        for _i in 0..num {
            stacks[from - 1].pop_back();
        }
    }

    tops(&stacks)
}

fn tops(stacks: &Vec<VecDeque<char>>) -> String {
    let mut res = String::with_capacity(stacks.len());
    for i in 0..stacks.len() {
        res.push(*stacks[i].back().unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::read_file;
    use crate::day5::{part1, part2};

    const EXAMPLE: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), "CMZ");
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, "MQTPGLLDN");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), "MCD");
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, "LVZPSTTCZ");
    }
}
