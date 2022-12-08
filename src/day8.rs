use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day8.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(iter: impl Iterator<Item = String>) -> Vec<Vec<u32>> {
    iter.map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

/// Visit each element in the matrix looking at it from one of the directions.
/// Calculate the cumulative max of the trees as we go along to compare against each new tree.
/// Record the tree coords that are visible in a set.
///
fn part1(input: impl Iterator<Item = String>) -> usize {
    let grid = parse_input(input);
    let dim = grid.len();
    let mut seen = HashSet::new();

    // From the left
    for row in 0..dim {
        let mut cum_max = -1;
        for col in 0..dim {
            let val = grid[row][col] as i32;
            if val > cum_max {
                seen.insert((row, col));
            }
            cum_max = cum_max.max(val)
        }
    }

    // From the right
    for row in 0..dim {
        let mut cum_max = -1;
        for col in (0..dim).rev() {
            let val = grid[row][col] as i32;
            if val > cum_max {
                seen.insert((row, col));
            }
            cum_max = cum_max.max(val)
        }
    }

    // From the top
    for col in 0..dim {
        let mut cum_max = -1;
        for row in 0..dim {
            let val = grid[row][col] as i32;
            if val > cum_max {
                seen.insert((row, col));
            }
            cum_max = cum_max.max(val)
        }
    }

    // From the bottom
    for col in 0..dim {
        let mut cum_max = -1;
        for row in (0..dim).rev() {
            let val = grid[row][col] as i32;
            if val > cum_max {
                seen.insert((row, col));
            }
            cum_max = cum_max.max(val)
        }
    }

    return seen.len();
}

/// From each tree, go in each of the four directions, stopping when we reach a tree of the same or greater height.
///
fn part2(input: impl Iterator<Item = String>) -> u32 {
    let grid = parse_input(input);
    let dim = grid.len();
    let mut max_score = 0;

    for row in 0..dim {
        for col in 0..dim {
            let val = grid[row][col];

            // Look left
            let mut left_score = 0;
            if col > 0 {
                for c in (0..col).rev() {
                    left_score += 1;
                    if grid[row][c] >= val {
                        break;
                    }
                }
            }

            // Look right
            let mut right_score = 0;
            for c in (col + 1)..dim {
                right_score += 1;
                if grid[row][c] >= val {
                    break;
                }
            }

            // Look up
            let mut up_score = 0;
            if row > 0 {
                for r in (0..row).rev() {
                    up_score += 1;
                    if grid[r][col] >= val {
                        break;
                    }
                }
            }

            // Look down
            let mut down_score = 0;
            for r in (row + 1)..dim {
                down_score += 1;
                if grid[r][col] >= val {
                    break;
                }
            }

            let score = left_score * right_score * up_score * down_score;
            max_score = max_score.max(score);
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 21);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 1798);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 8);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 259308);
    }
}
