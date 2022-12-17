use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::iter;

fn read_file() -> String {
    let mut file = File::open("input/day17.txt").unwrap();
    let mut line = String::new();
    file.read_to_string(&mut line).unwrap();
    line
}

type Coord = (i32, i32);

fn new_rock(rock_type: i32, left_x: i32, bottom_y: i32) -> Vec<Coord> {
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

fn part1(directions: String) -> i32 {
    let mut height = 0;
    let mut chamber = HashSet::new();
    let width = 7;
    let mut rock_type = 0;
    let mut dir_iter = directions.chars().cycle();

    for _ in 0..2022 {
        // for _ in 0..=15 {
        let mut rock = new_rock(rock_type, 2, height + 3);
        let mut turn = 0;
        loop {
            let dir = if turn % 2 == 0 {
                dir_iter.next().unwrap()
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
        // println!("Height: {}", height);
        // print_chamber(&chamber);
    }
    height
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

fn part2(_directions: String) -> u32 {
    unimplemented!()
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
        // assert_eq!(res, 0);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.to_string()), 3068);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
