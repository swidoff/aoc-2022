use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day2.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> u32 {
    let scores = HashMap::from([
        ("A X".to_string(), 1 + 3),
        ("A Y".to_string(), 2 + 6),
        ("A Z".to_string(), 3 + 0),
        ("B X".to_string(), 1 + 0),
        ("B Y".to_string(), 2 + 3),
        ("B Z".to_string(), 3 + 6),
        ("C X".to_string(), 1 + 6),
        ("C Y".to_string(), 2 + 0),
        ("C Z".to_string(), 3 + 3),
    ]);

    input.map(|s| scores.get(&s).unwrap()).sum()
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    let scores = HashMap::from([
        ("A X".to_string(), 3 + 0), // A - Rock, X - Lose => Scissors
        ("A Y".to_string(), 1 + 3), // A - Rock, Y - Draw => Rock
        ("A Z".to_string(), 2 + 6), // A - Rock, Z - Win => Paper
        ("B X".to_string(), 1 + 0), // B - Paper, X - Lose => Rock
        ("B Y".to_string(), 2 + 3), // B - Paper, Y - Draw => Paper
        ("B Z".to_string(), 3 + 6), // B - Paper, Z - Win => Scissors
        ("C X".to_string(), 2 + 0), // C - Scissors, X - Lose => Paper
        ("C Y".to_string(), 3 + 3), // C - Scissors, Y - Draw => Scissors
        ("C Z".to_string(), 1 + 6), // C - Scissors, Z - Win => Rock
    ]);

    input.map(|s| scores.get(&s).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2, read_file};

    const EXAMPLE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 15);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 13565);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 12);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 12424);
    }
}
