use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn read_file() -> String {
    let mut file = File::open("input/day6.txt").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    str
}

fn part1(input: String) -> usize {
    input
        .chars()
        .tuple_windows()
        .enumerate()
        .find_map(|(i, (c1, c2, c3, c4))| {
            if c1 != c2 && c1 != c3 && c1 != c4 && c2 != c3 && c2 != c4 && c3 != c4 {
                Some(i + 4)
            } else {
                None
            }
        })
        .unwrap()
}

fn part2(input: String) -> usize {
    // Safe because all chars are bytes here.
    for i in 14..input.len() + 1 {
        if input[i - 14..i].chars().all_unique() {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.to_string()), 5);
        assert_eq!(part1(EXAMPLE2.to_string()), 6);
        assert_eq!(part1(EXAMPLE3.to_string()), 10);
        assert_eq!(part1(EXAMPLE4.to_string()), 11);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 1042);
    }

    const EXAMPLE5: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE6: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE7: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE8: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE9: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE5.to_string()), 19);
        assert_eq!(part2(EXAMPLE6.to_string()), 23);
        assert_eq!(part2(EXAMPLE7.to_string()), 23);
        assert_eq!(part2(EXAMPLE8.to_string()), 29);
        assert_eq!(part2(EXAMPLE9.to_string()), 26);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 2980);
    }
}
