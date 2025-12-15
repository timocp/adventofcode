use crate::grid;
use crate::grid::Pos;
use ordered_float::NotNan;
use std::collections::HashSet;

pub struct Input {
    asteroids: Vec<Pos>,
    best_station_position: Pos,
    best_station_visible: u32,
}

pub fn parse_input(input: &str) -> Input {
    let asteroids = parse_asteroids(input);
    let (best_station_position, best_station_visible) = best_monitoring_station(&asteroids);
    Input {
        asteroids,
        best_station_position,
        best_station_visible,
    }
}

pub fn part1(input: &Input) -> u32 {
    input.best_station_visible
}

pub fn part2(input: &Input) -> i32 {
    let boom = each_vaporised_asteroid(input.best_station_position, &input.asteroids)
        .nth(199)
        .unwrap();
    boom.x * 100 + boom.y
}

fn best_monitoring_station(asteroids: &[Pos]) -> (Pos, u32) {
    asteroids
        .iter()
        .map(|&a| (a, count_visible(a, asteroids)))
        .max_by_key(|(_, count)| *count)
        .unwrap()
}

fn count_visible(from: Pos, asteroids: &[Pos]) -> u32 {
    asteroids
        .iter()
        .filter(|&&b| b != from)
        .map(|b| from.direction_dxdy(b))
        .collect::<HashSet<_>>()
        .len() as u32
}

#[derive(Debug)]
struct AsteroidState {
    p: Pos,
    destroyed: bool,
    // direction in degree
    direction: NotNan<f32>,
    // manhattan distance to other asteroid (manhattan is sufficient because we're
    // only comparing distances for asteroids in the same direction)
    distance: u32,
}

#[derive(Debug)]
struct VaporisationIter {
    asteroids: Vec<AsteroidState>,
    laser_angle: f32,
}

// iterator that returns each asteroid position in turn until there are none left
fn each_vaporised_asteroid(from: Pos, asteroids: &[Pos]) -> impl Iterator<Item = Pos> + '_ {
    VaporisationIter {
        asteroids: asteroids
            .iter()
            .filter(|&&p| p != from)
            .map(|p| AsteroidState {
                p: *p,
                destroyed: false,
                direction: NotNan::new(from.direction(p)).unwrap(),
                distance: from.manhattan_distance(p),
            })
            .collect(),
        laser_angle: 0f32.next_down(),
    }
}

impl Iterator for VaporisationIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match self
            .asteroids
            .iter()
            .enumerate()
            .filter(|(_i, a)| !a.destroyed)
            .min_by_key(|(_i, a)| {
                (
                    NotNan::new(rotation_angle(self.laser_angle, a.direction.into_inner()))
                        .unwrap(),
                    a.distance,
                )
            }) {
            Some((i, a)) => {
                self.laser_angle = a.direction.into_inner();
                self.asteroids[i].destroyed = true;
                Some(self.asteroids[i].p)
            }
            None => None,
        }
    }
}

// angle needed to rotate to hit target
// if already pointing at target, we need to rotate a full 360 to hit it
fn rotation_angle(laser: f32, target: f32) -> f32 {
    let mut angle = target - laser;
    if angle <= 0.0 {
        angle += 360.0
    }
    angle
}

fn parse_asteroids(input: &str) -> Vec<Pos> {
    grid::parse_each_char(input)
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
            (Pos { x, y }, count),
            best_monitoring_station(&parse_asteroids(example)),
        );
    }
}

#[test]
fn test_each_vaporised_asteroid() {
    let asteroids = parse_asteroids(
        "\
.#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##
",
    );
    let best = best_monitoring_station(&asteroids).0;
    assert_eq!(Pos::from((8, 3)), best);
    let destroyed: Vec<Pos> = each_vaporised_asteroid(best, &asteroids).collect();

    // first none to be vaporised
    assert_eq!(Pos::from((8, 1)), destroyed[0]);
    assert_eq!(Pos::from((9, 0)), destroyed[1]);
    assert_eq!(Pos::from((9, 1)), destroyed[2]);
    assert_eq!(Pos::from((10, 0)), destroyed[3]);
    assert_eq!(Pos::from((9, 2)), destroyed[4]);
    assert_eq!(Pos::from((11, 1)), destroyed[5]);
    assert_eq!(Pos::from((12, 1)), destroyed[6]);
    assert_eq!(Pos::from((11, 2)), destroyed[7]);
    assert_eq!(Pos::from((15, 1)), destroyed[8]);

    // last nine to be destroyed
    assert_eq!(Pos::from((6, 1)), destroyed[27]);
    assert_eq!(Pos::from((6, 0)), destroyed[28]);
    assert_eq!(Pos::from((7, 0)), destroyed[29]);
    assert_eq!(Pos::from((8, 0)), destroyed[30]);
    assert_eq!(Pos::from((10, 1)), destroyed[31]);
    assert_eq!(Pos::from((14, 0)), destroyed[32]);
    assert_eq!(Pos::from((16, 1)), destroyed[33]);
    assert_eq!(Pos::from((13, 3)), destroyed[34]);
    assert_eq!(Pos::from((14, 3)), destroyed[35]);
}
