use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    read_named_file("day10.txt")
}

fn read_named_file(name: &str) -> impl Iterator<Item = String> {
    let file = File::open(format!("input/{}", name)).unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let mut score = 0;

    for line in input {
        if (cycle - 20) % 40 == 0 {
            score += cycle * x;
        }
        cycle += 1;

        let words = line.split_ascii_whitespace().collect_vec();
        match words.as_slice() {
            ["noop"] => {}
            ["addx", val_str] => {
                if (cycle - 20) % 40 == 0 {
                    score += cycle * x;
                }
                cycle += 1;
                x += i32::from_str(val_str).unwrap();
            }
            _ => {}
        }
    }
    score
}

fn part2(input: impl Iterator<Item = String>) -> String {
    let mut pos = 0;
    let mut x: i32 = 1;
    let mut display = ['.'; 240];
    for line in input {
        let hor_pos = (pos % 40) as i32;
        if hor_pos >= x - 1 && hor_pos <= x + 1 {
            display[pos] = '#';
        }
        pos += 1;

        let words = line.split_ascii_whitespace().collect_vec();
        match words.as_slice() {
            ["noop"] => {}
            ["addx", val_str] => {
                let hor_pos = (pos % 40) as i32;
                if hor_pos >= x - 1 && hor_pos <= x + 1 {
                    display[pos] = '#';
                }
                pos += 1;
                x += i32::from_str(val_str).unwrap();
            }
            _ => {}
        }
    }

    let mut res = String::new();
    for i in 0..6 {
        res.push_str(format!("{}\n", &display[(i * 40)..((i + 1) * 40)].iter().join("")).as_str())
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};
    use crate::day10::read_named_file;

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(read_named_file("day10_example1.txt")), 13140);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        let actual = part2(read_named_file("day10_example1.txt"));
        println!("{}", actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);

        let answer = "###...##..#..#..##..####.###..####.####.
#..#.#..#.#.#..#..#.#....#..#.#.......#.
###..#....##...#..#.###..#..#.###....#..
#..#.#.##.#.#..####.#....###..#.....#...
#..#.#..#.#.#..#..#.#....#.#..#....#....
###...###.#..#.#..#.####.#..#.####.####.
";
        assert_eq!(res, answer);
    }
}
