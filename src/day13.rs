use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day13.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Scalar(u32),
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Packet> {
    input
        .batching(|iter| loop {
            let line = iter.next();
            match line {
                None => return None,
                Some(s) if s.is_empty() => {}
                Some(s) => return Some(parse_packet(s)),
            }
        })
        .collect_vec()
}

fn parse_packet(str: String) -> Packet {
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
                match iter.peek() {
                    Some(&v2) if v2.is_digit(10) => {
                        d = d * 10 + v2.to_digit(10).unwrap();
                        iter.next();
                    }
                    _ => {}
                }

                if let Some(Packet::List(ls)) = stack.back_mut() {
                    ls.push(Packet::Scalar(d))
                }
            }
        }
    }
    stack.pop_back().unwrap()
}

fn compare(p1: &Packet, p2: &Packet) -> Ordering {
    match (p1, p2) {
        (Packet::Scalar(v1), Packet::Scalar(v2)) => v1.cmp(v2),
        (Packet::List(ls1), Packet::List(ls2)) => {
            let mut i = 0;
            loop {
                if i == ls1.len() || i == ls2.len() {
                    return ls1.len().cmp(&ls2.len());
                }
                match compare(&ls1[i], &ls2[i]) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => {}
                }
                i += 1;
            }
        }
        (Packet::Scalar(v), p2) => compare(&Packet::List(vec![Packet::Scalar(*v)]), p2),
        (p1, Packet::Scalar(v)) => compare(p1, &Packet::List(vec![Packet::Scalar(*v)])),
    }
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let packets = parse_input(input);
    let mut res = 0;
    for (i, (p1, p2)) in packets.iter().tuples().enumerate() {
        if compare(p1, p2) == Ordering::Less {
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
    packets.sort_by(|p1, p2| compare(p1, p2));

    let mut res = 1;
    for (i, p) in packets.iter().enumerate() {
        if compare(p, &d1) == Ordering::Equal || compare(p, &d2) == Ordering::Equal {
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
