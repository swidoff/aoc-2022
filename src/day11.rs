use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day11.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

enum Operation {
    Times(u64),
    Plus(u64),
    Square(),
}

struct Monkey<ITEM> {
    items: VecDeque<ITEM>,
    op: Operation,
    test_denom: u64,
    true_monkey: usize,
    false_monkey: usize,
}

fn parse_input<F, ITEM>(lines: impl Iterator<Item = String>, mut init_item: F) -> Vec<Monkey<ITEM>>
where
    F: FnMut(u64) -> ITEM,
{
    lines
        .batching(|iter| {
            let mut iter = iter.peekable();
            if let None = iter.next() {
                return None;
            }
            let items = iter
                .next()
                .unwrap()
                .split_whitespace()
                .skip(2)
                .map(|v| {
                    let level = u64::from_str(v.trim_end_matches(',')).unwrap();
                    init_item(level)
                })
                .collect();

            let op = match iter
                .next()
                .unwrap()
                .split_whitespace()
                .skip(4)
                .collect_tuple()
                .unwrap()
            {
                ("*", "old") => Operation::Square(),
                ("*", n) => Operation::Times(u64::from_str(n).unwrap()),
                ("+", n) => Operation::Plus(u64::from_str(n).unwrap()),
                s => panic!("Unexpected operation: {:?}", s),
            };

            let test_denom = parse_last_num(&mut iter);
            let true_monkey = parse_last_num(&mut iter);
            let false_monkey = parse_last_num(&mut iter);

            if let Some(s) = iter.peek() {
                if s.is_empty() {
                    iter.next().unwrap();
                }
            }

            Some(Monkey {
                items,
                op,
                test_denom,
                true_monkey,
                false_monkey,
            })
        })
        .collect_vec()
}

fn parse_last_num<T: FromStr>(iter: &mut impl Iterator<Item = String>) -> T
where
    <T as FromStr>::Err: Debug,
{
    iter.next()
        .unwrap()
        .split_whitespace()
        .last()
        .map(|v| T::from_str(v).unwrap())
        .unwrap()
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let mut monkeys = parse_input(input, |v| v);
    let mut inspections = iter::repeat(0).take(monkeys.len()).collect_vec();
    for _round in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop_front() {
                inspections[i] += 1;

                let worry = match monkeys[i].op {
                    Operation::Times(v) => item * v,
                    Operation::Plus(v) => item + v,
                    Operation::Square() => item * item,
                } / 3;

                let target = if worry % monkeys[i].test_denom == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[target].items.push_back(worry);
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

struct Item {
    prime_remainders: Vec<u64>,
}

const PRIMES: [u64; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

impl Item {
    fn new(num: u64) -> Item {
        let mut prime_remainders = Vec::with_capacity(PRIMES.len());
        for prime in PRIMES {
            prime_remainders.push(num % prime)
        }
        Item { prime_remainders }
    }

    fn times(&mut self, other: u64) {
        for (i, rem) in self.prime_remainders.iter_mut().enumerate() {
            *rem = (*rem * other) % PRIMES[i];
        }
    }

    fn add(&mut self, other: u64) {
        for (i, rem) in self.prime_remainders.iter_mut().enumerate() {
            *rem = (*rem + other) % PRIMES[i];
        }
    }

    fn square(&mut self) {
        for (i, rem) in self.prime_remainders.iter_mut().enumerate() {
            *rem = (*rem * *rem) % PRIMES[i];
        }
    }

    fn is_divisible(&self, denom: u64) -> bool {
        for (i, &prime) in PRIMES.iter().enumerate() {
            if prime == denom {
                return self.prime_remainders[i] == 0;
            }
        }
        return false;
    }
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let mut monkeys = parse_input(input, |v| Item::new(v));
    let mut inspections = iter::repeat(0).take(monkeys.len()).collect_vec();
    for _round in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some(mut item) = monkeys[i].items.pop_front() {
                inspections[i] += 1;

                match monkeys[i].op {
                    Operation::Times(v) => item.times(v),
                    Operation::Plus(v) => item.add(v),
                    Operation::Square() => item.square(),
                };

                let target = if item.is_divisible(monkeys[i].test_denom) {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    inspections.sort();
    inspections[inspections.len() - 2] * inspections[inspections.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 10605);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 118674);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 2713310158);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 32333418600);
    }
}
