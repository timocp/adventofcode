use crate::Puzzle;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Solver {
    wire0: Wire,
    wire1: Wire,
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        Self {
            wire0: wire_from_str(lines[0]),
            wire1: wire_from_str(lines[1]),
        }
    }

    fn part1(&self) -> String {
        closest_intersection(&self.wire0, &self.wire1).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn closest_intersection(wire0: &Wire, wire1: &Wire) -> u32 {
    let seen0 = follow_seen(wire0);
    let mut distance = u32::MAX;

    follow(wire1, |pos| {
        if seen0.contains(&pos) {
            let this_distance = manhatten_distance(pos);
            if this_distance > 0 && this_distance < distance {
                distance = this_distance
            };
        }
    });

    distance
}

fn follow_seen(wire: &Wire) -> HashSet<(i32, i32)> {
    let mut seen = HashSet::new();
    follow(wire, |pos| {
        seen.insert(pos);
    });
    seen
}

fn follow(wire: &Wire, mut f: impl FnMut((i32, i32))) {
    let mut pos = (0, 0);
    f(pos);

    for step in wire {
        for _ in 0..step.len {
            pos = match step.dir {
                Dir::Up => (pos.0, pos.1 - 1),
                Dir::Right => (pos.0 + 1, pos.1),
                Dir::Down => (pos.0, pos.1 + 1),
                Dir::Left => (pos.0 - 1, pos.1),
            };
            f(pos);
        }
    }
}

fn manhatten_distance(pos: (i32, i32)) -> u32 {
    pos.0.abs() as u32 + pos.1.abs() as u32
}

#[derive(Debug, PartialEq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    len: u32,
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        let first = s.chars().next().unwrap();
        let len = s[1..].parse().unwrap();
        Step {
            dir: match first {
                'U' => Dir::Up,
                'R' => Dir::Right,
                'D' => Dir::Down,
                'L' => Dir::Left,
                _ => panic!("Unexpected dir: {}", first),
            },
            len,
        }
    }
}

type Wire = Vec<Step>;

fn wire_from_str(s: &str) -> Wire {
    s.split(",").map(Step::from).collect()
}

#[test]
fn test_input() {
    let test_input = "R8,U5,L5,D3\nU7,R6,D4,L4\n";
    let solver = Solver::new(test_input);
    assert_eq!(4, solver.wire0.len());
    assert_eq!(Dir::Right, solver.wire0[0].dir);
    assert_eq!(8, solver.wire0[0].len);
    assert_eq!(Dir::Down, solver.wire0[3].dir);
    assert_eq!(3, solver.wire0[3].len);
    assert_eq!(4, solver.wire1.len());
    assert_eq!(Dir::Up, solver.wire1[0].dir);
    assert_eq!(7, solver.wire1[0].len);
    assert_eq!(Dir::Left, solver.wire1[3].dir);
    assert_eq!(4, solver.wire1[3].len);
}

#[test]
fn test_closest_intersection() {
    assert_eq!(
        6,
        closest_intersection(&wire_from_str("R8,U5,L5,D3"), &wire_from_str("U7,R6,D4,L4"))
    );
    assert_eq!(
        159,
        closest_intersection(
            &wire_from_str("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &wire_from_str("U62,R66,U55,R34,D71,R55,D58,R83")
        )
    );
    assert_eq!(
        135,
        closest_intersection(
            &wire_from_str("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &wire_from_str("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        )
    );
}
