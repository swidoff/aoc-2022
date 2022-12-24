use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day24.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

const EMPTY: u8 = 0;
const LEFT: u8 = 1;
const RIGHT: u8 = 1 << 2;
const UP: u8 = 1 << 3;
const DOWN: u8 = 1 << 4;

type Coord = (i32, i32);

#[derive(Clone, Hash, Eq, PartialEq)]
struct State {
    blizzards: Vec<u8>,
    coord: Coord,
}

fn parse_input(input: impl Iterator<Item = String>, n_rows: i32, n_cols: i32) -> State {
    let mut blizzards = Vec::new();
    for (r, line) in input.enumerate() {
        if r == 0 || r == (n_rows + 1) as usize {
            continue;
        }
        for (c, ch) in line.chars().enumerate() {
            if c == 0 || c == (n_cols + 1) as usize {
                continue;
            }
            let value = match ch {
                '>' => RIGHT,
                '<' => LEFT,
                '^' => UP,
                'v' => DOWN,
                _ => EMPTY,
            };
            blizzards.push(value);
        }
    }
    State {
        blizzards,
        coord: (-1, 0),
    }
}

fn part1(
    input: impl Iterator<Item = String>,
    n_rows: i32,
    n_cols: i32,
    max_minutes: usize,
) -> usize {
    let initial_state = parse_input(input, n_rows, n_cols);
    let dest = (n_rows - 1 as i32, n_cols - 1 as i32);
    let mut memo = HashMap::new();
    let (_, minutes) = solve_quickest_path(
        initial_state,
        0,
        n_rows,
        n_cols,
        max_minutes,
        dest,
        &mut memo,
    );
    minutes
}

fn solve_quickest_path(
    state: State,
    minutes: usize,
    n_rows: i32,
    n_cols: i32,
    max_minutes: usize,
    dest: Coord,
    memo: &mut HashMap<State, (Option<State>, usize)>,
) -> (Option<State>, usize) {
    if let Some((final_state, solution)) = memo.get(&state) {
        return (final_state.clone(), *solution);
    }

    if minutes == max_minutes {
        return (None, usize::MAX);
    }

    if state.coord == dest {
        return (Some(state), minutes + 1);
    }

    let new_blizzards = advance(&state.blizzards, n_rows, n_cols);
    let (row, col) = state.coord;

    let mut best_state = None;
    let mut min_minutes = usize::MAX;
    for new_coord @ (new_row, new_col) in [
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
        (row, col),
    ] {
        if new_coord == state.coord
            || (new_row >= 0 && new_row < n_rows && new_col >= 0 && new_col < n_cols)
        {
            let i = new_row * n_cols + new_col;
            if i < 0 || i >= new_blizzards.len() as i32 || new_blizzards[i as usize] == EMPTY {
                let new_state = State {
                    blizzards: new_blizzards.clone(),
                    coord: new_coord,
                };
                let (final_state, new_minutes) = solve_quickest_path(
                    new_state,
                    minutes + 1,
                    n_rows,
                    n_cols,
                    max_minutes,
                    dest,
                    memo,
                );
                if new_minutes < min_minutes {
                    min_minutes = new_minutes;
                    best_state = final_state;
                }
            }
        }
    }

    memo.insert(state, (best_state.clone(), min_minutes));
    (best_state, min_minutes)
}

fn advance(blizzards: &Vec<u8>, n_rows: i32, n_cols: i32) -> Vec<u8> {
    let mut new_blizzards = iter::repeat(EMPTY).take(blizzards.len()).collect_vec();

    let n_rows = n_rows as usize;
    let n_cols = n_cols as usize;
    for (i, dirs) in blizzards.iter().enumerate() {
        let row = i / n_cols;
        let col = i % n_cols;
        if LEFT & dirs != 0 {
            let new_col = if col == 0 { n_cols - 1 } else { col - 1 };
            let new_i = row * n_cols + new_col;
            new_blizzards[new_i] |= LEFT;
        }
        if RIGHT & dirs != 0 {
            let new_col = if col == n_cols - 1 { 0 } else { col + 1 };
            let new_i = row * n_cols + new_col;
            new_blizzards[new_i] |= RIGHT;
        }
        if UP & dirs != 0 {
            let new_row = if row == 0 { n_rows - 1 } else { row - 1 };
            let new_i = new_row * n_cols + col;
            new_blizzards[new_i] |= UP;
        }
        if DOWN & dirs != 0 {
            let new_row = if row == n_rows - 1 { 0 } else { row + 1 };
            let new_i = new_row * n_cols + col;
            new_blizzards[new_i] |= DOWN;
        }
    }

    new_blizzards
}

fn part2(
    input: impl Iterator<Item = String>,
    n_rows: i32,
    n_cols: i32,
    max_minutes1: usize,
    max_minutes2: usize,
    max_minutes3: usize,
) -> usize {
    let initial_state = parse_input(input, n_rows, n_cols);
    let dest = (n_rows - 1 as i32, n_cols - 1 as i32);
    let mut memo = HashMap::new();
    let (state1, minutes1) = solve_quickest_path(
        initial_state,
        0,
        n_rows,
        n_cols,
        max_minutes1,
        dest,
        &mut memo,
    );
    memo.clear();
    println!("Minutes1: {}", minutes1);

    let state1 = State {
        blizzards: state1.unwrap().blizzards,
        coord: (n_rows, n_cols - 1),
    };

    let (state2, minutes2) =
        solve_quickest_path(state1, 0, n_rows, n_cols, max_minutes2, (0, 0), &mut memo);
    memo.clear();
    println!("Minutes2: {}", minutes2);

    let state2 = State {
        blizzards: state2.unwrap().blizzards,
        coord: (-1, 0),
    };

    let (_, minutes3) =
        solve_quickest_path(state2, 0, n_rows, n_cols, max_minutes3, dest, &mut memo);
    println!("Minutes3: {}", minutes3);

    minutes1 + minutes2 + minutes3 - 2
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "#E######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string()), 4, 6, 20), 18);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file(), 25, 120, 300);
        println!("{}", res);
        assert_eq!(res, 281);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(EXAMPLE.lines().map(|v| v.to_string()), 4, 6, 20, 25, 20),
            54
        );
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 25, 120, 300, 300, 300);
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
