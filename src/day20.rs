use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day20.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Id = usize;

fn parse_input(input: impl Iterator<Item = String>) -> Vec<i64> {
    input
        .map(|line| i64::from_str(line.as_str()).unwrap())
        .collect_vec()
}

fn part1(input: impl Iterator<Item = String>, zero_index: Id) -> i64 {
    let seq = parse_input(input);
    let mut res = (0..seq.len()).collect_vec();
    mix(&seq, &mut res);
    coordinates(&seq, &res, zero_index)
}

fn part2(input: impl Iterator<Item = String>, zero_index: usize) -> i64 {
    let key = 811589153;
    let seq = parse_input(input).iter().map(|&v| v * key).collect_vec();
    let mut res = (0..seq.len()).collect_vec();

    for _i in 0..10 {
        mix(&seq, &mut res);
    }
    coordinates(&seq, &res, zero_index)
}

fn coordinates(seq: &Vec<i64>, res: &Vec<Id>, zero_index: Id) -> i64 {
    let index = index_of(&res, zero_index);
    let v1 = seq[res[(index + 1000) as usize % res.len()]];
    let v2 = seq[res[(index + 2000) as usize % res.len()]];
    let v3 = seq[res[(index + 3000) as usize % res.len()]];
    v1 + v2 + v3
}

fn mix(seq: &Vec<i64>, res: &mut Vec<Id>) {
    for (id, &offset) in seq.iter().enumerate() {
        if offset == 0 {
            continue;
        }

        let index = index_of(&res, id);
        res.remove(index as usize);

        let len = res.len() as i64;
        let new_index = (index + offset) % len;
        if new_index >= 0 {
            res.insert(new_index as usize, id)
        } else {
            res.insert((len + new_index) as usize, id);
        }
    }
}

fn index_of(res: &Vec<Id>, target_id: Id) -> i64 {
    res.iter()
        .enumerate()
        .find_map(|(new_i, &id)| {
            if id == target_id {
                Some(new_i as i64)
            } else {
                None
            }
        })
        .unwrap()
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
        assert_eq!(res, 7865110481723);
    }
}
