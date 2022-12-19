use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (i32, i32, i32);
type Side = [Coord; 4];

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

fn count_sides(points: &Vec<Coord>) -> HashMap<Side, usize> {
    let mut sides = HashMap::new();
    for &(x, y, z) in points {
        let cube = sides_for_cube(x, y, z);

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

fn sides_for_cube(x: i32, y: i32, z: i32) -> [Side; 6] {
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
    let mut side1 = [points[0], points[1], points[2], points[3]];
    let mut side2 = [points[4], points[5], points[6], points[7]];
    let mut side3 = [points[2], points[3], points[6], points[7]];
    let mut side4 = [points[0], points[1], points[4], points[5]];
    let mut side5 = [points[0], points[2], points[4], points[6]];
    let mut side6 = [points[1], points[3], points[5], points[7]];
    side1.sort();
    side2.sort();
    side3.sort();
    side4.sort();
    side5.sort();
    side6.sort();
    let mut cube = [side1, side2, side3, side4, side5, side6];
    cube.sort();
    cube
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let cubes = parse_input(input);
    let mut sides: HashSet<Side> = count_sides(&cubes)
        .iter()
        .filter_map(|(side, count)| {
            if *count == 1 {
                Some(side.clone())
            } else {
                None
            }
        })
        .collect();

    let mut sides_seen = HashSet::new();
    let mut cubes_seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((1, 1, 1));

    while let Some(cube @ (x, y, z)) = q.pop_front() {
        if cubes_seen.contains(&cube) {
            continue;
        } else {
            cubes_seen.insert(cube.clone());
        }

        let air_sides = sides_for_cube(x, y, z);
        for air_side in air_sides.iter() {
            if sides.contains(air_side) {
                sides_seen.insert(air_side.clone());
            } else if x > -1 && air_side.iter().all(|&(sx, sy, sz)| sx == x - 1) {
                q.push_back((x - 1, y, z))
            } else if x < 25 && air_side.iter().all(|&(sx, sy, sz)| sx == x) {
                q.push_back((x + 1, y, z))
            } else if y > -1 && air_side.iter().all(|&(sx, sy, sz)| sy == y - 1) {
                q.push_back((x, y - 1, z))
            } else if y < 25 && air_side.iter().all(|&(sx, sy, sz)| sy == y) {
                q.push_back((x, y + 1, z))
            } else if z > -1 && air_side.iter().all(|&(sx, sy, sz)| sz == z - 1) {
                q.push_back((x, y, z - 1))
            } else if z < 25 && air_side.iter().all(|&(sx, sy, sz)| sz == z) {
                q.push_back((x, y, z + 1))
            }
        }
    }
    sides_seen.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};
    use crate::day18::sides_for_cube;

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
        // println!("{:?}", sides_for_cube(2, 2, 5));
        // [(1, 1, 4), (1, 1, 5), (1, 2, 4), (1, 2, 5)],
        // [(1, 1, 4), (1, 1, 5), (2, 1, 4), (2, 1, 5)],
        // [(1, 1, 4), (1, 2, 4), (2, 1, 4), (2, 2, 4)],
        // [(1, 1, 5), (1, 2, 5), (2, 1, 5), (2, 2, 5)],
        // [(1, 2, 4), (1, 2, 5), (2, 2, 4), (2, 2, 5)],
        // [(2, 1, 4), (2, 1, 5), (2, 2, 4), (2, 2, 5)]]
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 58);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 2456);
    }
}
