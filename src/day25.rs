use std::fs::File;
use std::io::{BufRead, BufReader};
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day25.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn snafu_to_decimal(snafu: String) -> i64 {
    let places = snafu.len();
    let mut res = 0;
    for (i, c) in snafu.chars().enumerate() {
        let digit = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        };
        let exp = (places - i - 1) as u32;
        res += digit * 5_i64.pow(exp);
    }
    res
}

fn decimal_to_snafu(dec: i64) -> String {
    let mut dec = dec;
    let mut res = String::new();
    while dec > 0 {
        let rem = dec % 5;
        let carry = if rem > 2 { 1 } else { 0 };
        dec = (dec - rem) / 5 + carry;
        let c = match rem {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "=",
            4 => "-",
            _ => panic!(),
        };
        res.push_str(c);
    }
    String::from_iter(res.chars().rev())
}

fn part1(input: impl Iterator<Item = String>) -> String {
    let res = input.map(|v| snafu_to_decimal(v)).sum();
    decimal_to_snafu(res)
}

#[cfg(test)]
mod tests {
    use super::{part1, read_file};
    use crate::day25::{decimal_to_snafu, snafu_to_decimal};
    use itertools::Itertools;
    use std::str::FromStr;

    const EXAMPLE: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    const CASES: &str = "        1              1
        2              2
        3             1=
        4             1-
        5             10
        6             11
        7             12
        8             2=
        9             2-
       10             20
       15            1=0
       20            1-0
     2022         1=11-2
    12345        1-0---0
314159265  1121-1110-1=0";

    #[test]
    fn test_part1_example() {
        for line in CASES.lines() {
            let (decimal_str, snafu) = line.split_whitespace().collect_tuple().unwrap();
            let decimal = i64::from_str(decimal_str).unwrap();
            assert_eq!(snafu_to_decimal(snafu.to_string()), decimal);
            assert_eq!(decimal_to_snafu(decimal), snafu);
        }

        assert_eq!(
            part1(EXAMPLE.lines().map(|v| v.to_string())),
            "2=-1=0".to_string()
        );
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, "2-=12=2-2-2-=0012==2".to_string());
    }
}
