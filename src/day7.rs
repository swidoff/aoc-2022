use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day7.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> HashMap<String, usize> {
    let mut stack = VecDeque::new();
    let mut dirs = HashMap::new();

    for line in input {
        let v = line.split(" ").collect_vec();
        match v.as_slice() {
            ["$", "cd", ".."] => {
                stack.pop_back().unwrap();
            }
            ["$", "cd", dir] => {
                let mut path = stack.iter().join("/");
                path.extend(dir.chars());
                stack.push_back(path.clone());
                dirs.insert(path, 0);
            }
            ["$", "ls"] => {}
            ["dir", _dir_name] => {}
            [num, _name] => {
                let size = usize::from_str(*num).unwrap();
                for dir in stack.iter() {
                    *dirs.get_mut(dir).unwrap() += size;
                }
            }
            _ => {}
        }
    }
    dirs
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let dirs = parse_input(input);
    dirs.values().filter(|&&size| size <= 100000).sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let dirs = parse_input(input);
    let used_space = dirs.get(&"/".to_string()).unwrap();
    let free_space = 70_000_000 - used_space;
    let needed_space = 30_000_000 - free_space;

    *dirs
        .values()
        .filter(|&&size| size >= needed_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 95437);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 1391690);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 24933642);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 5469168);
    }
}
