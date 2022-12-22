use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day22.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (usize, usize);

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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
            parse_instructions(&line, &mut instructions);
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

fn parse_instructions(line: &String, instructions: &mut Vec<Instruction>) {
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

type Coord3d = (usize, usize, usize);

struct Map3d {
    walls: HashSet<Coord3d>,
    instructions: Vec<Instruction>,
}

#[derive(Copy, Clone)]
enum Entrance {
    Top,
    Bottom,
    Left,
    Right,
}

struct Side {
    row_offset: usize,
    col_offset: usize,
    neighbors: HashMap<Dir, (usize, Entrance)>,
}

fn parse_input_3d(input: impl Iterator<Item = String>, dim: usize) -> Map3d {
    let mut walls = HashSet::new();
    let mut instructions = Vec::new();
    let mut parse_directions = false;

    for (row, line) in input.enumerate() {
        if line.is_empty() {
            parse_directions = true;
            continue;
        }

        if parse_directions {
            parse_instructions(&line, &mut instructions);
        } else {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    walls.insert((row / dim, row % dim, col));
                }
            }
        }
    }

    Map3d {
        walls,
        instructions,
    }
}

fn part2(input: impl Iterator<Item = String>, dim: usize, sides: [Side; 6]) -> usize {
    let map = parse_input_3d(input, dim);
    let mut side = 0;
    let mut row = 0;
    let mut col = 0;
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
            &Instruction::Move(n) => {
                for _i in 0..n {
                    if let Some((new_dir, new_side, new_row, new_col)) =
                        move1(dir, side, row, col, dim, &sides, &map.walls)
                    {
                        dir = new_dir;
                        side = new_side;
                        row = new_row;
                        col = new_col;
                        println!(
                            "{:?}, {}, {}, {}",
                            dir,
                            side + 1,
                            row + 1 + sides[side].row_offset,
                            col + 1 + sides[side].col_offset
                        );
                    } else {
                        break;
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

    1000 * (row + sides[side].row_offset + 1) + 4 * (col + sides[side].col_offset + 1) + facing
}

fn move1(
    dir: Dir,
    side: usize,
    row: usize,
    col: usize,
    dim: usize,
    sides: &[Side; 6],
    walls: &HashSet<Coord3d>,
) -> Option<(Dir, usize, usize, usize)> {
    let (new_dir, new_side, new_row, new_col) = match dir {
        Dir::Right => {
            if col == dim - 1 {
                let &(new_side, entrance) = sides[side].neighbors.get(&Dir::Right).unwrap();
                let (new_dir, new_row, new_col) = change_sides(Dir::Right, entrance, row, col, dim);
                (new_dir, new_side, new_row, new_col)
            } else {
                (dir, side, row, col + 1)
            }
        }
        Dir::Left => {
            if col == 0 {
                let &(new_side, entrance) = sides[side].neighbors.get(&Dir::Left).unwrap();
                let (new_dir, new_row, new_col) = change_sides(Dir::Left, entrance, row, col, dim);
                (new_dir, new_side, new_row, new_col)
            } else {
                (dir, side, row, col - 1)
            }
        }
        Dir::Down => {
            if row == dim - 1 {
                let &(new_side, entrance) = sides[side].neighbors.get(&Dir::Down).unwrap();
                let (new_dir, new_row, new_col) = change_sides(Dir::Down, entrance, row, col, dim);
                (new_dir, new_side, new_row, new_col)
            } else {
                (dir, side, row + 1, col)
            }
        }
        Dir::Up => {
            if row == 0 {
                let &(new_side, entrance) = sides[side].neighbors.get(&Dir::Up).unwrap();
                let (new_dir, new_row, new_col) = change_sides(Dir::Up, entrance, row, col, dim);
                (new_dir, new_side, new_row, new_col)
            } else {
                (dir, side, row - 1, col)
            }
        }
    };

    if walls.contains(&(new_side, new_row, new_col)) {
        None
    } else {
        Some((new_dir, new_side, new_row, new_col))
    }
}

fn change_sides(
    dir: Dir,
    entrance: Entrance,
    row: usize,
    col: usize,
    dim: usize,
) -> (Dir, usize, usize) {
    match (dir, entrance) {
        (Dir::Right, Entrance::Right) => (Dir::Left, dim - row - 1, dim - 1),
        (Dir::Right, Entrance::Top) => (Dir::Down, 0, dim - row - 1),
        (Dir::Right, Entrance::Bottom) => panic!(),
        (Dir::Right, Entrance::Left) => (Dir::Right, row, 0),
        (Dir::Left, Entrance::Right) => (Dir::Left, row, dim - 1),
        (Dir::Left, Entrance::Top) => (Dir::Down, 0, row),
        (Dir::Left, Entrance::Bottom) => (Dir::Up, dim - 1, dim - row - 1),
        (Dir::Left, Entrance::Left) => panic!(),
        (Dir::Up, Entrance::Right) => (Dir::Left, dim - col - 1, dim - 1),
        (Dir::Up, Entrance::Top) => (Dir::Down, 0, dim - col - 1),
        (Dir::Up, Entrance::Bottom) => (Dir::Up, dim - 1, col),
        (Dir::Up, Entrance::Left) => (Dir::Right, col, 0),
        (Dir::Down, Entrance::Right) => panic!(),
        (Dir::Down, Entrance::Top) => (Dir::Down, 0, col),
        (Dir::Down, Entrance::Bottom) => (Dir::Up, dim - 1, dim - col - 1),
        (Dir::Down, Entrance::Left) => (Dir::Right, dim - col - 1, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};
    use crate::day22::{Dir, Entrance, Side};
    use std::collections::HashMap;

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

    const EXAMPLE2: &str = "...#
.#..
#...
....
...#
....
..#.
....
....     
....    
...#    
....    
...#
#...
....
..#.
...#
....
.#..
....    
....    
.#..    
....        
..#.    

10R5L5R10L4R5L5
";

    #[test]
    fn test_part2_example() {
        let sides = [
            Side {
                // 1
                row_offset: 0,
                col_offset: 4 * 2,
                neighbors: HashMap::from([
                    (Dir::Up, (2 - 1, Entrance::Top)),
                    (Dir::Right, (6 - 1, Entrance::Right)),
                    (Dir::Down, (4 - 1, Entrance::Top)),
                    (Dir::Left, (3 - 1, Entrance::Top)),
                ]),
            },
            Side {
                // 2
                row_offset: 4,
                col_offset: 0,
                neighbors: HashMap::from([
                    (Dir::Up, (1 - 1, Entrance::Top)),
                    (Dir::Right, (3 - 1, Entrance::Left)),
                    (Dir::Down, (5 - 1, Entrance::Bottom)),
                    (Dir::Left, (6 - 1, Entrance::Bottom)),
                ]),
            },
            Side {
                // 3
                row_offset: 4,
                col_offset: 4,
                neighbors: HashMap::from([
                    (Dir::Up, (1 - 1, Entrance::Left)),
                    (Dir::Right, (4 - 1, Entrance::Left)),
                    (Dir::Down, (5 - 1, Entrance::Left)),
                    (Dir::Left, (2 - 1, Entrance::Right)),
                ]),
            },
            Side {
                // 4
                row_offset: 4,
                col_offset: 4 * 2,
                neighbors: HashMap::from([
                    (Dir::Up, (1 - 1, Entrance::Bottom)),
                    (Dir::Right, (6 - 1, Entrance::Top)),
                    (Dir::Down, (5 - 1, Entrance::Top)),
                    (Dir::Left, (3 - 1, Entrance::Right)),
                ]),
            },
            Side {
                // 5
                row_offset: 4 * 2,
                col_offset: 4 * 2,
                neighbors: HashMap::from([
                    (Dir::Up, (4 - 1, Entrance::Bottom)),
                    (Dir::Right, (6 - 1, Entrance::Left)),
                    (Dir::Down, (2 - 1, Entrance::Bottom)),
                    (Dir::Left, (3 - 1, Entrance::Bottom)),
                ]),
            },
            Side {
                // 6
                row_offset: 4 * 2,
                col_offset: 4 * 3,
                neighbors: HashMap::from([
                    (Dir::Up, (4 - 1, Entrance::Right)),
                    (Dir::Right, (1 - 1, Entrance::Right)),
                    (Dir::Down, (2 - 1, Entrance::Left)),
                    (Dir::Left, (5 - 1, Entrance::Right)),
                ]),
            },
        ];
        assert_eq!(
            part2(EXAMPLE2.lines().map(|v| v.to_string()), 4, sides),
            5031
        );
    }

    // #[test]
    // fn test_part2() {
    //     let res = part2(read_file());
    //     println!("{}", res);
    // assert_eq!(res, 0);
    // }
}
