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
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
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

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }
}

struct State {
    direction: Direction,
    position: (i32, i32),
}

impl State {
    fn new() -> Self {
        Self {
            direction: Direction::North,
            position: (0, 0),
        }
    }

    fn turn(&mut self, turn: Turn) {
        self.direction = self.direction.turn(turn);
    }

    fn walk(&mut self, walk: u32) {
        match self.direction {
            Direction::North => self.position.1 += walk as i32,
            Direction::East => self.position.0 += walk as i32,
            Direction::South => self.position.1 -= walk as i32,
            Direction::West => self.position.0 -= walk as i32,
        };
    }

    fn distance_from_origin(&self) -> u32 {
        self.position.0.unsigned_abs() + self.position.1.unsigned_abs()
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
