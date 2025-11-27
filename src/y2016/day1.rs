use crate::grid::{Compass, ORIGIN, Pos};
use std::collections::HashSet;

pub struct Solver {
    instructions: Vec<Instruction>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            instructions: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        part1(&self.instructions).to_string()
    }

    fn part2(&self) -> String {
        part2(&self.instructions).to_string()
    }
}

fn part1(input: &[Instruction]) -> u32 {
    let mut state = State::new();
    for instruction in input {
        state.turn(instruction.turn);
        state.walk(instruction.walk);
    }
    state.distance_from_origin()
}

fn part2(input: &[Instruction]) -> u32 {
    let mut state = State::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    for instruction in input {
        state.turn(instruction.turn);
        for _ in 0..instruction.walk {
            state.walk(1);
            if visited.contains(&state.position) {
                return state.distance_from_origin();
            }
            visited.insert(state.position);
        }
    }
    panic!("Didn't visit any location twice")
}

struct State {
    direction: Compass,
    position: Pos,
}

impl State {
    fn new() -> Self {
        Self {
            direction: Compass::North,
            position: ORIGIN,
        }
    }

    fn turn(&mut self, turn: Turn) {
        self.direction = match turn {
            Turn::Left => self.direction.left90(),
            Turn::Right => self.direction.right90(),
        }
    }

    fn walk(&mut self, walk: u32) {
        self.position = self.position.walk(self.direction, walk as i32);
    }

    fn distance_from_origin(&self) -> u32 {
        self.position.manhattan_distance(&ORIGIN)
    }
}

#[derive(Clone, Copy)]
enum Turn {
    Left,
    Right,
}

struct Instruction {
    turn: Turn,
    walk: u32,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim_end_matches('\n')
        .split(", ")
        .map(|instruction| Instruction {
            turn: match instruction.chars().next().unwrap() {
                'L' => Turn::Left,
                'R' => Turn::Right,
                _ => panic!("Invalid turn"),
            },
            walk: instruction[1..].parse().unwrap(),
        })
        .collect()
}

#[test]
fn test() {
    assert_eq!(5, part1(&parse_input("R2, L3")));
    assert_eq!(2, part1(&parse_input("R2, R2, R2")));
    assert_eq!(12, part1(&parse_input("R5, L5, R5, R3")));

    assert_eq!(4, part2(&parse_input("R8, R4, R4, R8")));
}
