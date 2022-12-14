use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day3.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|rucksack| {
            let len = rucksack.len();
            let (compartment1, compartment2) = rucksack.split_at(len / 2);
            sum(bitset(compartment1) & bitset(compartment2))
        })
        .sum()
}

/// Find the 0-56 index of a char.
///
fn index(char: char) -> u64 {
    if char.is_ascii_uppercase() {
        u64::from(char) - u64::from('A') + 26
    } else {
        u64::from(char) - u64::from('a')
    }
}

/// Set a bit for each char in the string. They are encoded 0-56.
///
fn bitset(str: &str) -> u64 {
    let mut bitset = 0;
    for c in str.chars() {
        bitset |= 1 << index(c);
    }
    bitset
}

/// Sum the bits in the bit set.
///
fn sum(bitset: u64) -> u64 {
    let mut score = 0;
    let mut bitset = bitset;
    while bitset > 0 {
        bitset >>= 1;
        score += 1;
    }
    score
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .tuples()
        .map(|(bag1, bag2, bag3)| sum(bitset(&bag1[..]) & bitset(&bag2[..]) & bitset(&bag3[..])))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2, read_file};

    const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 157);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 8085);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 70);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 2515);
    }
}
