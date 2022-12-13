use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day13.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    List(Vec<Packet>),
    Scalar(u32),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Scalar(v1), Packet::Scalar(v2)) => v1.cmp(v2),
            (Packet::List(ls1), Packet::List(ls2)) => {
                let mut i = 0;
                loop {
                    if i == ls1.len() || i == ls2.len() {
                        return ls1.len().cmp(&ls2.len());
                    }
                    match ls1[i].cmp(&ls2[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                    i += 1;
                }
            }
            (Packet::Scalar(v), p2) => Packet::List(vec![Packet::Scalar(*v)]).cmp(p2),
            (p1, Packet::Scalar(v)) => p1.cmp(&Packet::List(vec![Packet::Scalar(*v)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl From<String> for Packet {
    fn from(str: String) -> Self {
        let mut stack = VecDeque::new();
        let mut iter = str.chars().peekable();
        while let Some(char) = iter.next() {
            match char {
                '[' => stack.push_back(Packet::List(Vec::new())),
                ']' => {
                    if stack.len() > 1 {
                        let packet = stack.pop_back().unwrap();
                        if let Some(Packet::List(ls)) = stack.back_mut() {
                            ls.push(packet)
                        }
                    }
                }
                ',' => {}
                v => {
                    let mut d = v.to_digit(10).unwrap();
                    while let Some(c) = iter.peek() {
                        if let Some(d2) = c.to_digit(10) {
                            d = d * 10 + d2;
                            iter.next();
                        } else {
                            break;
                        }
                    }
                    if let Some(Packet::List(ls)) = stack.back_mut() {
                        ls.push(Packet::Scalar(d))
                    }
                }
            }
        }
        stack.pop_back().unwrap()
    }
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Packet> {
    input
        .batching(|iter| loop {
            let line = iter.next();
            match line {
                None => return None,
                Some(s) if s.is_empty() => {}
                Some(s) => return Some(Packet::from(s)),
            }
        })
        .collect_vec()
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let packets = parse_input(input);
    let mut res = 0;
    for (i, (p1, p2)) in packets.iter().tuples().enumerate() {
        if p1.cmp(p2) == Ordering::Less {
            res += i + 1;
        }
    }
    res
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let mut packets = parse_input(input);
    let d1 = Packet::List(vec![Packet::List(vec![Packet::Scalar(2)])]);
    let d2 = Packet::List(vec![Packet::List(vec![Packet::Scalar(6)])]);
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort();

    let mut res = 1;
    for (i, p) in packets.iter().enumerate() {
        if p.cmp(&d1) == Ordering::Equal || p.cmp(&d2) == Ordering::Equal {
            res *= i + 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 13);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 5503);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 140);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 20952);
    }
}
