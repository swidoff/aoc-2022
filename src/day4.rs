use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day4.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|s| {
            let (min1, max1, min2, max2) = parse_ranges(s);
            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count()
}

fn parse_ranges(s: &String) -> (u32, u32, u32, u32) {
    let (range1, range2) = s.split_once(",").unwrap();
    let (min1, max1) = parse_range(range1);
    let (min2, max2) = parse_range(range2);
    (min1, max1, min2, max2)
}

fn parse_range(range: &str) -> (u32, u32) {
    let (min, max) = range.split_once("-").unwrap();
    let min = u32::from_str(min).unwrap();
    let max = u32::from_str(max).unwrap();
    (min, max)
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    input
        .filter(|s| {
            let (min1, max1, min2, max2) = parse_ranges(s);
            max1 >= min2 && max2 >= min1
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 2);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 538);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 4);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 792);
    }
}
