use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day20.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<i64> {
    input
        .map(|line| i64::from_str(line.as_str()).unwrap())
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

fn part1(input: impl Iterator<Item = String>, zero_index: usize) -> i64 {
    let seq = parse_input(input);
    let mut res = (0..seq.len()).collect_vec();
    println!("{} {}", res.len(), res.iter().unique().count());

    mix(&seq, &mut res);
    coordinates(&seq, &res, zero_index)
}

fn coordinates(seq: &Vec<i64>, res: &Vec<usize>, zero_index: usize) -> i64 {
    let index = index_of(&res, zero_index);
    let v1 = seq[res[(index + 1000) as usize % res.len()]];
    let v2 = seq[res[(index + 2000) as usize % res.len()]];
    let v3 = seq[res[(index + 3000) as usize % res.len()]];
    println!("{} {} {} {}", v1, v2, v3, res.iter().unique().count());
    v1 + v2 + v3
}

fn mix(seq: &Vec<i64>, res: &mut Vec<usize>) {
    for (id, &e) in seq.iter().enumerate() {
        let index = index_of(&res, id);

        let mut new_index = if e < 0 {
            index - (e.abs() % seq.len() as i64)
        } else {
            (index + e + 1) % seq.len() as i64
        };

        if new_index < 0 {
            new_index = seq.len() as i64 + new_index;
        }

        if new_index != index {
            if new_index > index {
                res.insert(new_index as usize, id);
                res.remove(index as usize);
            } else {
                res.remove(index as usize);
                res.insert(new_index as usize, id);
            }
        }
    }
}

fn index_of(res: &Vec<usize>, target_id: usize) -> i64 {
    let index = res
        .iter()
        .enumerate()
        .find_map(|(new_i, &id)| {
            if id == target_id {
                Some(new_i as i64)
            } else {
                None
            }
        })
        .unwrap();
    index
}

fn part2(input: impl Iterator<Item = String>, zero_index: usize) -> i64 {
    let key = 811589153;
    let seq = parse_input(input);
    let decoder_seq = seq.iter().map(|&v| v * key).collect_vec();
    let mut res = (0..seq.len()).collect_vec();
    println!("{} {}", res.len(), res.iter().unique().count());

    println!(
        "Before: {:?}",
        res.iter().map(|&id| decoder_seq[id]).collect_vec()
    );

    for i in 0..10 {
        mix(&decoder_seq, &mut res);
        println!(
            "After {}: {:?}",
            i + 1,
            res.iter().map(|&id| decoder_seq[id]).collect_vec()
        );
    }

    coordinates(&decoder_seq, &res, zero_index)
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
        assert_eq!(res, 8372);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string()), 5), 1623178306);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 4094);
        println!("{}", res);
        // assert_eq!(res, 0);
        // Too low: 7284824237328
    }
}
