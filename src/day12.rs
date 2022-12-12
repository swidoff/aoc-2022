use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day12.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (i32, i32);

fn parse_input(iter: impl Iterator<Item = String>) -> (Vec<Vec<i32>>, Coord, Coord) {
    let mut grid = Vec::new();
    let mut start_coord = (0, 0);
    let mut end_coord = (0, 0);
    for (row, line) in iter.enumerate() {
        let mut vec = Vec::new();
        for (col, c) in line.chars().enumerate() {
            let value = match c {
                'S' => {
                    start_coord = (row as i32, col as i32);
                    0
                }
                'E' => {
                    end_coord = (row as i32, col as i32);
                    25
                }
                c => (u32::from(c) - u32::from('a')) as i32,
            };
            vec.push(value);
        }
        grid.push(vec);
    }

    (grid, start_coord, end_coord)
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    steps: i32,
    coord: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.steps).cmp(&other.steps).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: impl Iterator<Item = String>) -> i32 {
    let (grid, start_coord, end_coord) = parse_input(input);
    shortest_path(grid, vec![start_coord], end_coord)
}

fn shortest_path(grid: Vec<Vec<i32>>, start_coords: Vec<Coord>, end_coord: Coord) -> i32 {
    let n_rows = grid.len() as i32;
    let n_cols = grid[0].len() as i32;
    let mut distances = HashMap::new();
    let mut q = BinaryHeap::new();
    for start_coord in start_coords.iter() {
        let initial_state = State {
            steps: 0,
            coord: *start_coord,
        };
        q.push(initial_state);
    }

    while let Some(State {
        steps,
        coord: coord @ (row, col),
    }) = q.pop()
    {
        if coord == end_coord {
            return steps;
        }
        if let Some(&old_steps) = distances.get(&coord) {
            if old_steps < steps {
                distances.insert(coord, steps);
            } else {
                continue;
            }
        } else {
            distances.insert(coord, steps);
        }

        let c = grid[row as usize][col as usize];
        for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_row = row + dr;
            let new_col = col + dc;
            if new_row >= 0 && new_row < n_rows && new_col >= 0 && new_col < n_cols {
                let next_c = grid[new_row as usize][new_col as usize];
                if next_c <= c + 1 {
                    let new_coord = (new_row, new_col);
                    q.push(State {
                        steps: steps + 1,
                        coord: new_coord,
                    });
                }
            }
        }
    }
    0
}

fn part2(input: impl Iterator<Item = String>) -> i32 {
    let (grid, _, end_coord) = parse_input(input);
    let mut start_coords = Vec::new();
    for r in 0..grid.len() {
        let row = &grid[r];
        for c in 0..row.len() {
            if grid[r][c] == 0 {
                start_coords.push((r as i32, c as i32));
            }
        }
    }

    shortest_path(grid, start_coords, end_coord)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 31);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 447);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 29);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 446);
    }
}
