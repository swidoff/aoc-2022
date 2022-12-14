use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day14.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (usize, usize);

fn parse_input(input: impl Iterator<Item = String>) -> (HashSet<Coord>, usize) {
    let mut grid = HashSet::new();
    let mut max_y = 0;

    for line in input {
        for (coord1_str, coord2_str) in line.split(" -> ").tuple_windows() {
            let (x1, y1) = parse_coord(coord1_str);
            let (x2, y2) = parse_coord(coord2_str);
            let start_x = x1.min(x2);
            let end_x = x1.max(x2);
            let start_y = y1.min(y2);
            let end_y = y1.max(y2);

            for x in start_x..(end_x + 1) {
                for y in start_y..(end_y + 1) {
                    grid.insert((x, y));
                }
            }
            max_y = max_y.max(y1).max(y2);
        }
    }

    (grid, max_y)
}

fn parse_coord(coord_str: &str) -> Coord {
    let (x_str, y_str) = coord_str.split(",").collect_tuple().unwrap();
    let x = usize::from_str(x_str).unwrap();
    let y = usize::from_str(y_str).unwrap();
    (x, y)
}

fn part1(input: impl Iterator<Item = String>) -> u32 {
    let (mut grid, max_y) = parse_input(input);
    let mut sand = 0;
    while let Some(sand_coord) = drop_sand(&grid, max_y, usize::MAX) {
        grid.insert(sand_coord);
        sand += 1;
    }
    return sand;
}

fn drop_sand(grid: &HashSet<Coord>, max_iterations: usize, max_y: usize) -> Option<Coord> {
    let mut x = 500;
    let mut y = 0;
    let mut iterations = 0;
    while iterations < max_iterations {
        if y == max_y {
            return Some((x, y));
        } else {
            if !grid.contains(&(x, y + 1)) {
                y += 1;
            } else if !grid.contains(&(x - 1, y + 1)) {
                y += 1;
                x -= 1;
            } else if !grid.contains(&(x + 1, y + 1)) {
                y += 1;
                x += 1;
            } else {
                return Some((x, y));
            }
        }
        iterations += 1;
    }
    None
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    let (mut grid, max_y) = parse_input(input);
    let mut sand = 0;
    while let Some(sand_coord) = drop_sand(&grid, usize::MAX, max_y + 1) {
        grid.insert(sand_coord);
        sand += 1;
        if sand_coord == (500, 0) {
            break;
        }
    }
    return sand;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 24);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 644);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 93);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 27324);
    }
}
