use crate::grid;
use crate::grid::P;
use gcd::Gcd;
use std::collections::HashSet;

pub struct Solver {
    asteroids: Vec<P>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            asteroids: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        best_monitoring_station(&self.asteroids).1.to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn best_monitoring_station(asteroids: &[P]) -> (P, u32) {
    asteroids
        .iter()
        .map(|&a| (a, count_visible(a, asteroids)))
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn count_visible(a: P, asteroids: &[P]) -> u32 {
    asteroids
        .iter()
        .filter(|&&b| b != a)
        .map(|&b| direction(a, b))
        .collect::<HashSet<_>>()
        .len() as u32
}

fn direction(from: P, to: P) -> (i32, i32) {
    if from == to {
        panic!("Can't determine direction from {} to self", from);
    }
    let d = to - from;
    let gcd = d.x.unsigned_abs().gcd(d.y.unsigned_abs()) as i32;
    (d.x / gcd, d.y / gcd)
}

fn parse_input(input: &str) -> Vec<P> {
    grid::each_char(input)
        .filter_map(|(p, c)| if c == '#' { Some(p) } else { None })
        .collect()
}

#[test]
fn test_best_monitoring_station() {
    for (x, y, count, example) in [
        (
            3,
            4,
            8,
            "\
.#..#
.....
#####
....#
...##
",
        ),
        (
            5,
            8,
            33,
            "\
......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####
",
        ),
        (
            1,
            2,
            35,
            "\
#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.
",
        ),
        (
            6,
            3,
            41,
            "\
.#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..
",
        ),
        (
            11,
            13,
            210,
            "\
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##
",
        ),
    ] {
        assert_eq!(
            (P { x, y }, count),
            best_monitoring_station(&parse_input(example)),
        );
    }
}
