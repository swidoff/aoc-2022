use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (i32, i32, i32);

fn parse_input(iter: impl Iterator<Item = String>) -> Vec<Coord> {
    iter.map(|s| {
        s.split(",")
            .map(|c| i32::from_str(c).unwrap())
            .collect_tuple()
            .unwrap()
    })
    .collect_vec()
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let points = parse_input(input);
    let sides = count_sides(&points);
    sides.values().filter(|&&v| v == 1).sum()
}

fn count_sides(points: &Vec<Coord>) -> HashMap<[Coord; 4], usize> {
    let mut sides = HashMap::new();
    for &(x, y, z) in points {
        let points = [
            (x - 1, y - 1, z - 1),
            (x, y - 1, z - 1),
            (x - 1, y, z - 1),
            (x, y, z - 1),
            (x - 1, y - 1, z),
            (x, y - 1, z),
            (x - 1, y, z),
            (x, y, z),
        ];
        let cube = [
            [points[0], points[1], points[2], points[3]],
            [points[4], points[5], points[6], points[7]],
            [points[2], points[3], points[6], points[7]],
            [points[0], points[1], points[4], points[5]],
            [points[0], points[2], points[4], points[6]],
            [points[1], points[3], points[5], points[7]],
        ];

        for side in cube {
            if let Some(count) = sides.get_mut(&side) {
                *count += 1;
            } else {
                sides.insert(side, 1);
            }
        }
    }
    sides
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    let cubes = parse_input(input);
    let sides = count_sides(&cubes);
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "1,1,1
2,1,1";

    const EXAMPLE2: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 10);
        assert_eq!(part1(EXAMPLE2.lines().map(|v| v.to_string())), 64);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 4320);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 58);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
