use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day1.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn elf_calories(input: impl Iterator<Item = String>) -> impl Iterator<Item = u64> {
    input.chain(iter::once("".to_string())).batching(|it| {
        let mut sum = 0;
        loop {
            match it.next() {
                None => return None,
                Some(str) if str.is_empty() => return Some(sum),
                Some(calories) => sum += u64::from_str(calories.as_str()).unwrap(),
            }
        }
    })
}

fn max_elf_calories(input: impl Iterator<Item = String>) -> u64 {
    elf_calories(input).max().unwrap()
}

fn top_3_elf_calories(input: impl Iterator<Item = String>) -> u64 {
    elf_calories(input).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::day1::{max_elf_calories, read_file, top_3_elf_calories};

    const EXAMPLE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn part1_example() {
        let input = EXAMPLE.lines().map(|s| s.to_string());
        assert_eq!(max_elf_calories(input), 24000);
    }

    #[test]
    fn part1() {
        let input = read_file();
        let res = max_elf_calories(input);
        println!("{}", res);
        assert_eq!(res, 72017);
    }

    #[test]
    fn part2_example() {
        let input = EXAMPLE.lines().map(|s| s.to_string());
        assert_eq!(top_3_elf_calories(input), 45000);
    }

    #[test]
    fn part2() {
        let input = read_file();
        let res = top_3_elf_calories(input);
        println!("{}", res);
        assert_eq!(res, 212520);
    }
}
