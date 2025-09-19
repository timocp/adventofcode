use crate::Part;
use std::collections::HashSet;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input),
        }
    )
}

#[derive(Debug)]
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

fn deliver_presents<'a>(houses: &mut HashSet<(i32, i32)>, dirs: impl Iterator<Item = &'a Dir>) {
    let mut pos = (0, 0); // x,y
    houses.insert(pos);
    for dir in dirs {
        pos = match dir {
            Dir::North => (pos.0, pos.1 - 1),
            Dir::East => (pos.0 + 1, pos.1),
            Dir::South => (pos.0, pos.1 + 1),
            Dir::West => (pos.0 - 1, pos.1),
        };
        houses.insert(pos);
    }
}

fn part1(dirs: &[Dir]) -> usize {
    let mut houses = HashSet::new();
    deliver_presents(&mut houses, dirs.iter());
    houses.len()
}

fn part2(dirs: &[Dir]) -> usize {
    let mut houses = HashSet::new();
    deliver_presents(&mut houses, dirs.iter().step_by(2));
    deliver_presents(&mut houses, dirs.iter().skip(1).step_by(2));
    houses.len()
}

#[test]
fn test() {
    assert_eq!(2, part1(&parse_input(">")));
    assert_eq!(4, part1(&parse_input("^>v<")));
    assert_eq!(2, part1(&parse_input("^v^v^v^v^v")));

    assert_eq!(3, part2(&parse_input("^>")));
    assert_eq!(3, part2(&parse_input("^>v<")));
    assert_eq!(11, part2(&parse_input("^v^v^v^v^v")));
}
