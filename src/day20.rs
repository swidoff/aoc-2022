use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day20.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<i32> {
    input
        .map(|line| i32::from_str(line.as_str()).unwrap())
        .collect_vec()
}

#[derive(PartialEq, Clone)]
struct Index(f32);

impl Eq for Index {}

impl PartialOrd<Self> for Index {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn part1(input: impl Iterator<Item = String>, zero_index: usize) -> i32 {
    let seq = parse_input(input);
    let mut res = seq.iter().enumerate().map(|(i, v)| (i, *v)).collect_vec();
    println!("{} {}", res.len(), res.iter().unique().count());

    for (i, &e) in seq.iter().enumerate() {
        let index = index_of(&res, e, i);

        let mut new_index = if e < 0 {
            index - (e.abs() % seq.len() as i32)
        } else {
            (index + e + 1) % seq.len() as i32
        };

        if new_index < 0 {
            new_index = seq.len() as i32 + new_index;
        }

        if new_index != index {
            if new_index > index {
                res.insert(new_index as usize, (i, e));
                res.remove(index as usize);
            } else {
                res.remove(index as usize);
                res.insert(new_index as usize, (i, e));
            }
        }
        // println!("After {}: {:?}", e, res)
    }

    let index = index_of(&res, 0, zero_index);
    let v1 = res[(index + 1000) as usize % res.len()];
    let v2 = res[(index + 2000) as usize % res.len()];
    let v3 = res[(index + 3000) as usize % res.len()];
    println!("{} {} {} {}", v1.1, v2.1, v3.1, res.iter().unique().count());
    v1.1 + v2.1 + v3.1
}

fn index_of(res: &Vec<(usize, i32)>, e: i32, i: usize) -> i32 {
    let index = res
        .iter()
        .enumerate()
        .find_map(|(new_i, &(old_i, v))| {
            if v == e && old_i == i {
                Some(new_i as i32)
            } else {
                None
            }
        })
        .unwrap();
    index
}

fn part2(_input: impl Iterator<Item = String>) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string()), 5), 3);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file(), 4094);
        println!("{}", res);
        // assert_eq!(res, 0);
        // Too low: 5277
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
