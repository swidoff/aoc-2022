use itertools::Itertools;

type Coord = (i64, i64);

fn distance((x1, y1): &Coord, (x2, y2): &Coord) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(input: &[(Coord, Coord)], y: i64) -> u32 {
    let max_distance = input.iter().map(|(c1, c2)| distance(c1, c2)).max().unwrap();
    let &min_x = input
        .iter()
        .map(|((x1, _y1), (x2, _y2))| x1.min(x2))
        .min()
        .unwrap();
    let &max_x = input
        .iter()
        .map(|((x1, _y1), (x2, _y2))| x1.max(x2))
        .max()
        .unwrap();
    let distances = input
        .iter()
        .map(|(sc, bc)| (*sc, *bc, distance(sc, bc)))
        .collect_vec();

    let mut count = 0;
    for x in (min_x - max_distance)..(max_x + max_distance + 1) {
        let c = (x, y);
        for (sc, bc, dist) in distances.iter() {
            if c != *bc && distance(sc, &c) <= *dist {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(input: &[(Coord, Coord)], max: i64) -> i64 {
    let distances = input
        .iter()
        .map(|(sc, bc)| (*sc, *bc, distance(sc, bc)))
        .collect_vec();

    // Since there is only one point where the sensor ranges do not overlap, that point must be a distance of one
    // away from at least one of the sensors. Gather the points around the border of each sensors ranges and try
    // those. There are way fewer than 4M x 4M points.
    for ((x, y), _bc, d) in distances.iter() {
        let d = d + 1;
        for dx in 0..(d + 1) {
            let dy = d - dx;
            for c @ (x, y) in [
                (x + dx, y + dy),
                (x - dx, y + dy),
                (x - dx, y - dy),
                (x + dx, y - dy),
            ] {
                if x >= 0
                    && x <= max
                    && y >= 0
                    && y <= max
                    && distances
                        .iter()
                        .map(|(sc, _bc, dist)| distance(sc, &c) > *dist)
                        .all(|v| v)
                {
                    return x * 4_000_000 + y;
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::day15::Coord;

    const EXAMPLE: [(Coord, Coord); 14] = [
        ((2, 18), (-2, 15)),
        ((9, 16), (10, 16)),
        ((13, 2), (15, 3)),
        ((12, 14), (10, 16)),
        ((10, 20), (10, 16)),
        ((14, 17), (10, 16)),
        ((8, 7), (2, 10)),
        ((2, 0), (2, 10)),
        ((0, 11), (2, 10)),
        ((20, 14), (25, 17)),
        ((17, 20), (21, 22)),
        ((16, 7), (15, 3)),
        ((14, 3), (15, 3)),
        ((20, 1), (15, 3)),
    ];

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(&EXAMPLE, 10), 26);
    }

    const INPUT: [(Coord, Coord); 33] = [
        ((2302110, 2237242), (2348729, 1239977)),
        ((47903, 2473047), (-432198, 2000000)),
        ((2363579, 1547888), (2348729, 1239977)),
        ((3619841, 520506), (2348729, 1239977)),
        ((3941908, 3526118), (3772294, 3485243)),
        ((3206, 1564595), (-432198, 2000000)),
        ((3123411, 3392077), (2977835, 3592946)),
        ((3279053, 3984688), (2977835, 3592946)),
        ((2968162, 3938490), (2977835, 3592946)),
        ((1772120, 2862246), (2017966, 3158243)),
        ((3283241, 2619168), (3172577, 2521434)),
        ((2471642, 3890150), (2977835, 3592946)),
        ((3163348, 3743489), (2977835, 3592946)),
        ((2933313, 2919047), (3172577, 2521434)),
        ((2780640, 3629927), (2977835, 3592946)),
        ((3986978, 2079918), (3998497, 2812428)),
        ((315464, 370694), (-550536, 260566)),
        ((3957316, 3968366), (3772294, 3485243)),
        ((2118533, 1074658), (2348729, 1239977)),
        ((3494855, 3378533), (3772294, 3485243)),
        ((2575727, 210553), (2348729, 1239977)),
        ((3999990, 2813525), (3998497, 2812428)),
        ((3658837, 3026912), (3998497, 2812428)),
        ((1551619, 1701155), (2348729, 1239977)),
        ((2625855, 3330422), (2977835, 3592946)),
        ((3476946, 2445098), (3172577, 2521434)),
        ((2915568, 1714113), (2348729, 1239977)),
        ((729668, 3723377), (-997494, 3617758)),
        ((3631681, 3801747), (3772294, 3485243)),
        ((2270816, 3197807), (2017966, 3158243)),
        ((3999999, 2810929), (3998497, 2812428)),
        ((3978805, 3296024), (3772294, 3485243)),
        ((1054910, 811769), (2348729, 1239977)),
    ];

    #[test]
    fn test_part1() {
        let res = part1(&INPUT, 2000000);
        println!("{}", res);
        assert_eq!(res, 5083287);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(&EXAMPLE, 20), 56000011);
    }

    #[test]
    fn test_part2() {
        let res = part2(&INPUT, 4_000_000);
        println!("{}", res);
        assert_eq!(res, 13134039205729);
    }
}
