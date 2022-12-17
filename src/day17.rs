use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::iter;
use std::iter::Cycle;
use std::os::linux::raw::stat;
use std::str::Chars;
use std::str::FromStr;

fn read_file() -> String {
    let mut file = File::open("input/day17.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    line
}

type Coord = (i64, i64);

fn new_rock(rock_type: i32, left_x: i64, bottom_y: i64) -> Vec<Coord> {
    match rock_type {
        0 => (0..4).map(|i| (left_x + i, bottom_y)).collect_vec(),
        1 => vec![
            (left_x, bottom_y + 1),
            (left_x + 1, bottom_y + 1),
            (left_x + 2, bottom_y + 1),
            (left_x + 1, bottom_y),
            (left_x + 1, bottom_y + 2),
        ],
        2 => vec![
            (left_x + 2, bottom_y + 2),
            (left_x + 2, bottom_y + 1),
            (left_x + 2, bottom_y),
            (left_x + 1, bottom_y),
            (left_x, bottom_y),
        ],
        3 => (0..4).map(|i| (left_x, bottom_y + i)).collect_vec(),
        4 => vec![
            (left_x, bottom_y),
            (left_x + 1, bottom_y),
            (left_x, bottom_y + 1),
            (left_x + 1, bottom_y + 1),
        ],
        _ => panic!(),
    }
}

fn part1(directions: String) -> i64 {
    let initial_state = State {
        rock_type: 0,
        dir_pos: 0,
        chamber: "".to_string(),
    };
    solve(&initial_state, &directions, 2022).extra_height
}

fn print_chamber(chamber: &HashSet<Coord>) {
    let height = chamber.iter().map(|(_x, y)| *y).max().unwrap();
    for y in (0..=height).rev() {
        print!("|");
        for x in 0..7 {
            if chamber.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("+-------+");
    println!();
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    rock_type: i32,
    dir_pos: usize,
    chamber: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Solution {
    new_state: State,
    extra_height: i64,
    num_rocks: i64,
}

fn solve(state: &State, directions: &String, num_rocks: i64) -> Solution {
    let mut chamber = restore(&state.chamber);
    let initial_height = chamber.iter().map(|&(_x, y)| y + 1).max().unwrap_or(0);
    let mut height = chamber.iter().map(|&(_x, y)| y).max().unwrap_or(0);
    let width = 7;
    let mut rock_type = state.rock_type;
    let mut dir_iter = directions.chars().enumerate().cycle().skip(state.dir_pos);
    let mut dir_pos = state.dir_pos;

    for _i in 0..num_rocks {
        let mut rock = new_rock(rock_type, 2, height + 3);
        let mut turn = 0;
        loop {
            let dir = if turn % 2 == 0 {
                let (pos, dir) = dir_iter.next().unwrap();
                dir_pos = pos + 2;
                dir
            } else {
                'v'
            };
            turn = (turn + 1) % 2;

            rock = match dir {
                'v' => {
                    let new_rock = rock.iter().map(|&(x, y)| (x, y - 1)).collect_vec();
                    if new_rock
                        .iter()
                        .any(|c @ (_x, y)| *y < 0 || chamber.contains(c))
                    {
                        break;
                    }
                    new_rock
                }
                c @ '<' | c @ '>' => {
                    let dx = if c == '>' { 1 } else { -1 };
                    let new_rock = rock.iter().map(|&(x, y)| (x + dx, y)).collect_vec();
                    if new_rock
                        .iter()
                        .any(|c @ (x, _y)| *x < 0 || *x >= width || chamber.contains(c))
                    {
                        continue;
                    }
                    new_rock
                }
                _ => panic!(),
            };
        }

        for c @ (_x, y) in rock {
            chamber.insert(c);
            height = height.max(y + 1);
        }

        rock_type = (rock_type + 1) % 5;
    }
    Solution {
        new_state: State {
            rock_type,
            dir_pos,
            chamber: memoize(&chamber),
        },
        extra_height: height - initial_height,
        num_rocks,
    }
}

fn part2(directions: String) -> i64 {
    let mut rocks_remaining: i64 = 1_000_000_000_000;
    let mut height = 0;
    let initial_state = State {
        rock_type: 0,
        dir_pos: 0,
        chamber: "".to_string(),
    };
    let mut state = State {
        rock_type: 0,
        dir_pos: 0,
        chamber: "".to_string(),
    };

    while rocks_remaining > 0 {
        let solution = solve(&state, &directions, 1_000);

        if solution.new_state == state {
            let repeat = rocks_remaining / solution.num_rocks;
            height += solution.extra_height * repeat;
            rocks_remaining -= solution.num_rocks * repeat;
            break;
        } else {
            height += solution.extra_height;
            rocks_remaining -= solution.num_rocks;
            state = solution.new_state.clone();
        }
    }
    let sol1 = solve(&initial_state, &directions, 1000);
    let sol2 = solve(&sol1.new_state, &directions, 1000);
    let sol3 = solve(&initial_state, &directions, 2000);
    assert_eq!(sol2.new_state, sol3.new_state);
    assert_eq!(sol1.extra_height + sol2.extra_height, sol3.extra_height);

    height
}

fn memoize(chamber: &HashSet<Coord>) -> String {
    let mut buf = String::new();
    let max_height = chamber.iter().map(|(_x, y)| *y).max().unwrap_or(0);
    let min_height = (max_height - 20).max(0) + 1;
    for y in (min_height..=max_height).rev() {
        buf.push('|');
        for x in 0..7 {
            if chamber.contains(&(x, y)) {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        buf.push('|');
        buf.push('\n');
    }
    buf
}

fn restore(memo: &String) -> HashSet<Coord> {
    let mut res = HashSet::new();

    for (y, line) in memo.lines().rev().enumerate() {
        for (x, c) in line.chars().dropping_back(1).skip(1).enumerate() {
            if c == '#' {
                res.insert((x as i64, (y + 1) as i64));
            }
        }
    }
    res
}
#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.to_string()), 3068);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 3109);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.to_string()), 1514285714288);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
