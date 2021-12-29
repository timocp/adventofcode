use crate::Part;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input),
            Part::Two => 0,
        }
    )
}

enum Dir {
    North,
    East,
    South,
    West,
}

impl From<char> for Dir {
    fn from(c: char) -> Self {
        match c {
            '^' => Dir::North,
            '>' => Dir::East,
            'v' => Dir::South,
            '<' => Dir::West,
            _ => panic!("unexpected direction: {}", c),
        }
    }
}

fn parse_input(input: &str) -> Vec<Dir> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(Dir::from)
        .collect()
}

fn deliver_presents(dirs: &[Dir]) -> HashMap<(i32, i32), usize> {
    let mut delivered = HashMap::new();
    let mut pos = (0, 0); // x,y
    delivered.insert(pos, 1);
    for dir in dirs {
        pos = match dir {
            Dir::North => (pos.0, pos.1 - 1),
            Dir::East => (pos.0 + 1, pos.1),
            Dir::South => (pos.0, pos.1 + 1),
            Dir::West => (pos.0 - 1, pos.1),
        };
        *delivered.entry(pos).or_insert(0) += 1;
    }
    delivered
}

fn part1(dirs: &[Dir]) -> usize {
    deliver_presents(dirs).len()
}

#[test]
fn test() {
    assert_eq!(2, part1(&parse_input(">")));
    assert_eq!(4, part1(&parse_input("^>v<")));
    assert_eq!(2, part1(&parse_input("^v^v^v^v^v")));
}
