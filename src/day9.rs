use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day9.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

#[derive(Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input(input: impl Iterator<Item = String>) -> impl Iterator<Item = Dir> {
    input.flat_map(|line| {
        let (dir_str, num_str) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let dir = match dir_str {
            "L" => Dir::Left,
            "R" => Dir::Right,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!("Unknown direction {}", dir_str),
        };
        let num = usize::from_str(num_str).unwrap();
        iter::repeat(dir).take(num)
    })
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let mut visited = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    visited.insert(tail);

    for dir in parse_input(input) {
        head = move_head(&head, dir);
        tail = move_tail(&head, &tail);
        visited.insert(tail);
    }

    visited.len()
}

fn move_head(head: &(i32, i32), dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Left => (head.0, head.1 - 1),
        Dir::Right => (head.0, head.1 + 1),
        Dir::Up => (head.0 - 1, head.1),
        Dir::Down => (head.0 + 1, head.1),
    }
}

fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    let r_diff = head.0 - tail.0;
    let c_diff = head.1 - tail.1;
    let r_dist = r_diff.abs();
    let c_dist = c_diff.abs();

    if !(r_dist > 1 || c_dist > 1) {
        *tail
    } else {
        let tail_r = if r_dist > 0 {
            tail.0 + r_diff.signum()
        } else {
            tail.0
        };

        let tail_c = if c_dist > 0 {
            tail.1 + c_diff.signum()
        } else {
            tail.1
        };

        (tail_r, tail_c)
    }
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let mut visited = HashSet::new();
    let mut knots = iter::repeat((0, 0)).take(10).collect_vec();

    for dir in parse_input(input) {
        knots[0] = move_head(&knots[0], dir);
        for i in 1..knots.len() {
            knots[i] = move_tail(&knots[i - 1], &knots[i]);
        }
        visited.insert(knots[knots.len() - 1]);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 13);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 6209);
    }

    const EXAMPLE2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 36);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 2460);
    }
}
