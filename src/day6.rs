use crate::Part;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run(input: &str, part: Part) -> String {
    let instructions = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&instructions),
            Part::Two => 0,
        }
    )
}

#[derive(Debug)]
enum Action {
    TurnOn,
    Toggle,
    TurnOff,
}

impl From<&str> for Action {
    fn from(s: &str) -> Self {
        match s {
            "turn on" => Action::TurnOn,
            "toggle" => Action::Toggle,
            "turn off" => Action::TurnOff,
            _ => panic!("unexpected action: {}", s),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    from: (u32, u32), // x,y
    to: (u32, u32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
        }
        let m = RE.captures(s).unwrap();
        Self {
            action: Action::from(&m[1]),
            from: (m[2].parse().unwrap(), m[3].parse().unwrap()),
            to: (m[4].parse().unwrap(), m[5].parse().unwrap()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn build_axes(index: Vec<(u32, u32)>) -> Vec<u32> {
    let mut numbers: Vec<u32> = index
        .iter()
        .map(|p| vec![p.0, p.1 + 1])
        .flatten()
        .collect::<HashSet<u32>>()
        .into_iter()
        .collect();
    numbers.sort_unstable();
    numbers
}

fn part1(instructions: &[Instruction]) -> usize {
    let x_axes = build_axes(
        instructions
            .iter()
            .map(|inst| (inst.from.0, inst.to.0))
            .collect(),
    );
    let y_axes = build_axes(
        instructions
            .iter()
            .map(|inst| (inst.from.1, inst.to.1))
            .collect(),
    );

    let x_map: HashMap<u32, usize> =
        HashMap::from_iter(x_axes.iter().enumerate().map(|(i, c)| (*c, i)));
    let y_map: HashMap<u32, usize> =
        HashMap::from_iter(y_axes.iter().enumerate().map(|(i, c)| (*c, i)));

    let mut grid = vec![vec![false; y_axes.len()]; x_axes.len()];

    for inst in instructions {
        let x0 = x_map[&inst.from.0];
        let x1 = x_map[&(inst.to.0 + 1)];
        let y0 = y_map[&inst.from.1];
        let y1 = y_map[&(inst.to.1 + 1)];
        for x in x0..x1 {
            for y in y0..y1 {
                grid[x][y] = match inst.action {
                    Action::TurnOn => true,
                    Action::Toggle => !grid[x][y],
                    Action::TurnOff => false,
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..(x_axes.len() - 1) {
        for y in 0..(y_axes.len() - 1) {
            if grid[x][y] {
                count += (x_axes[x + 1] - x_axes[x]) as usize * (y_axes[y + 1] - y_axes[y]) as usize
            }
        }
    }
    count
}

#[test]
fn test() {
    assert_eq!(
        1000 * 1000,
        part1(&parse_input("turn on 0,0 through 999,999\n"))
    );
    assert_eq!(1000, part1(&parse_input("toggle 0,0 through 999,0\n")));
    assert_eq!(0, part1(&parse_input("turn off 499,499 through 500,500\n")));
}
