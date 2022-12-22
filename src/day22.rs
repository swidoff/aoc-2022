use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day22.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (usize, usize);

#[derive(Eq, PartialEq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

enum Instruction {
    TurnLeft,
    TurnRight,
    Move(u32),
}

struct Map {
    walls: HashSet<Coord>,
    row_limits: Vec<[usize; 2]>,
    col_limits: Vec<[usize; 2]>,
    instructions: Vec<Instruction>,
}

fn parse_input(input: impl Iterator<Item = String>) -> Map {
    let mut walls = HashSet::new();
    let mut row_limits = Vec::new();
    let mut col_limits = Vec::new();
    let mut instructions = Vec::new();
    let mut parse_directions = false;

    for (row, line) in input.enumerate() {
        if line.is_empty() {
            parse_directions = true;
            continue;
        }

        if parse_directions {
            let mut num = 0;
            for c in line.chars() {
                if c.is_digit(10) {
                    num = num * 10 + c.to_digit(10).unwrap();
                } else {
                    instructions.push(Instruction::Move(num));
                    if c == 'L' {
                        instructions.push(Instruction::TurnLeft);
                    } else {
                        instructions.push(Instruction::TurnRight);
                    }
                    num = 0;
                }
            }
            instructions.push(Instruction::Move(num));
        } else {
            let mut row_limit = [usize::MAX, usize::MIN];
            for (col, c) in line.chars().enumerate() {
                if col == col_limits.len() {
                    col_limits.push([usize::MAX, usize::MIN]);
                }

                if c != ' ' {
                    row_limit[0] = row_limit[0].min(col);
                    row_limit[1] = row_limit[1].max(col);
                    col_limits[col][0] = col_limits[col][0].min(row);
                    col_limits[col][1] = col_limits[col][1].max(row);
                }
                if c == '#' {
                    walls.insert((row, col));
                }
            }

            row_limits.push(row_limit);
        }
    }

    Map {
        walls,
        row_limits,
        col_limits,
        instructions,
    }
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let map = parse_input(input);
    let mut row = 0;
    let mut col = map.row_limits[row][0];
    let mut dir = Dir::Right;

    for instruction in map.instructions.iter() {
        match instruction {
            Instruction::TurnLeft if dir == Dir::Left => dir = Dir::Down,
            Instruction::TurnLeft if dir == Dir::Right => dir = Dir::Up,
            Instruction::TurnLeft if dir == Dir::Up => dir = Dir::Left,
            Instruction::TurnLeft if dir == Dir::Down => dir = Dir::Right,
            Instruction::TurnRight if dir == Dir::Left => dir = Dir::Up,
            Instruction::TurnRight if dir == Dir::Right => dir = Dir::Down,
            Instruction::TurnRight if dir == Dir::Up => dir = Dir::Right,
            Instruction::TurnRight if dir == Dir::Down => dir = Dir::Left,
            &Instruction::Move(n) if dir == Dir::Right => {
                for _i in 0..n {
                    let new_col = if col == map.row_limits[row][1] {
                        map.row_limits[row][0]
                    } else {
                        col + 1
                    };
                    if map.walls.contains(&(row, new_col)) {
                        break;
                    } else {
                        col = new_col;
                    }
                }
            }
            &Instruction::Move(n) if dir == Dir::Left => {
                for _i in 0..n {
                    let new_col = if col == map.row_limits[row][0] {
                        map.row_limits[row][1]
                    } else {
                        col - 1
                    };
                    if map.walls.contains(&(row, new_col)) {
                        break;
                    } else {
                        col = new_col;
                    }
                }
            }
            &Instruction::Move(n) if dir == Dir::Down => {
                for _i in 0..n {
                    let new_row = if row == map.col_limits[col][1] {
                        map.col_limits[col][0]
                    } else {
                        row + 1
                    };
                    if map.walls.contains(&(new_row, col)) {
                        break;
                    } else {
                        row = new_row;
                    }
                }
            }
            &Instruction::Move(n) if dir == Dir::Up => {
                for _i in 0..n {
                    let new_row = if row == map.col_limits[col][0] {
                        map.col_limits[col][1]
                    } else {
                        row - 1
                    };
                    if map.walls.contains(&(new_row, col)) {
                        break;
                    } else {
                        row = new_row;
                    }
                }
            }
            _ => panic!(),
        }
    }

    let facing = match dir {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3,
    };

    1000 * (row + 1) + 4 * (col + 1) + facing
}

fn part2(_input: impl Iterator<Item = String>) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 6032);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 93226);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 0);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
