use itertools::Itertools;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day7.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Directory {
    files: Vec<Fyle>,
    dirs: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    fn flatten(&self) -> Vec<Rc<RefCell<Directory>>> {
        let mut res = Vec::new();
        for dir in self.dirs.iter() {
            res.push(dir.clone());
            res.extend(dir.borrow().flatten());
        }
        res
    }

    fn total_size(&self) -> usize {
        let file_sizes: usize = self.files.iter().map(|f| f.size).sum();
        let dir_sizes: usize = self.dirs.iter().map(|d| d.borrow().total_size()).sum();
        file_sizes + dir_sizes
    }
}

struct Fyle {
    size: usize,
}

fn parse_input(input: impl Iterator<Item = String>) -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory {
        files: Vec::new(),
        dirs: Vec::new(),
        parent: None,
    }));
    let mut cwd = root.clone();

    for line in input.skip(1) {
        let v = line.split(" ").collect_vec();
        match v.as_slice() {
            ["$", "cd", ".."] => {
                let parent = cwd.borrow().parent.as_ref().unwrap().clone();
                cwd = parent;
            }
            ["$", "cd", _dir] => {
                let new_dir = Rc::new(RefCell::new(Directory {
                    files: Vec::new(),
                    dirs: Vec::new(),
                    parent: Some(cwd.clone()),
                }));
                cwd.borrow_mut().dirs.push(new_dir.clone());
                cwd = new_dir;
            }
            ["$", "ls"] => {}
            ["dir", _dir_name] => {}
            [num, _name] => {
                let fyle = Fyle {
                    size: usize::from_str(*num).unwrap(),
                };
                cwd.borrow_mut().files.push(fyle);
            }
            _ => {}
        }
    }
    root
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let root_dir = parse_input(input);
    let root_dir = root_dir.borrow();
    root_dir
        .flatten()
        .iter()
        .map(|d| d.borrow().total_size())
        .filter(|&size| size <= 100000)
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let root_dir = parse_input(input);
    let root_dir = root_dir.borrow();
    let used_space = root_dir.total_size();
    let free_space = 70_000_000 - used_space;
    let needed_space = 30_000_000 - free_space;

    root_dir
        .flatten()
        .iter()
        .map(|d| d.borrow().total_size())
        .filter(|&size| size >= needed_space)
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
