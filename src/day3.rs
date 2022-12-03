use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day3.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> u32 {
    input
        .map(|rucksack| {
            let len = rucksack.len();
            let (compartment1, compartment2) = rucksack.split_at(len / 2);
            let compartment1: HashSet<char> = HashSet::from_iter(compartment1.chars());
            let compartment2 = HashSet::from_iter(compartment2.chars());
            let mut intersection = compartment1.intersection(&compartment2);
            let &char = intersection.next().unwrap();
            priority(char)
        })
        .sum()
}

fn priority(char: char) -> u32 {
    if char.is_ascii_uppercase() {
        u32::from(char) - u32::from('A') + 27
    } else {
        u32::from(char) - u32::from('a') + 1
    }
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    input
        .tuples()
        .map(|(bag1, bag2, bag3)| {
            let bag1: HashSet<char> = HashSet::from_iter(bag1.chars());
            let bag2: HashSet<char> = HashSet::from_iter(bag2.chars());
            let bag3: HashSet<char> = HashSet::from_iter(bag3.chars());
            for i in bag1.iter() {
                if bag2.contains(i) && bag3.contains(&i) {
                    return priority(*i);
                }
            }
            0
        })
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
