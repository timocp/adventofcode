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
            Part::Two => part2(&instructions),
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

fn build_axes(instructions: &[Instruction]) -> (Vec<u32>, Vec<u32>) {
    let mut x = HashSet::new();
    let mut y = HashSet::new();
    for inst in instructions {
        x.insert(inst.from.0);
        x.insert(inst.to.0 + 1);
        y.insert(inst.from.1);
        y.insert(inst.to.1 + 1);
    }
    let mut x: Vec<_> = x.into_iter().collect();
    let mut y: Vec<_> = y.into_iter().collect();
    x.sort_unstable();
    y.sort_unstable();
    (x, y)
}

fn build_axes_map(axes: &[u32]) -> HashMap<u32, usize> {
    HashMap::from_iter(axes.iter().enumerate().map(|(i, c)| (*c, i)))
}

#[allow(clippy::needless_range_loop)]
fn part1(instructions: &[Instruction]) -> usize {
    let (x_axes, y_axes) = build_axes(instructions);
    let x_map = build_axes_map(&x_axes);
    let y_map = build_axes_map(&y_axes);

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

#[allow(clippy::needless_range_loop)]
fn part2(instructions: &[Instruction]) -> usize {
    let (x_axes, y_axes) = build_axes(instructions);
    let x_map = build_axes_map(&x_axes);
    let y_map = build_axes_map(&y_axes);

    let mut grid = vec![vec![0; y_axes.len()]; x_axes.len()];

    for inst in instructions {
        let x0 = x_map[&inst.from.0];
        let x1 = x_map[&(inst.to.0 + 1)];
        let y0 = y_map[&inst.from.1];
        let y1 = y_map[&(inst.to.1 + 1)];
        for x in x0..x1 {
            for y in y0..y1 {
                match inst.action {
                    Action::TurnOn => grid[x][y] += 1,
                    Action::Toggle => grid[x][y] += 2,
                    Action::TurnOff => {
                        if grid[x][y] > 0 {
                            grid[x][y] -= 1;
                        }
                    }
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..(x_axes.len() - 1) {
        for y in 0..(y_axes.len() - 1) {
            count += (x_axes[x + 1] - x_axes[x]) as usize
                * (y_axes[y + 1] - y_axes[y]) as usize
                * grid[x][y]
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

    assert_eq!(1, part2(&parse_input("turn on 0,0 through 0,0\n")));
    assert_eq!(2000000, part2(&parse_input("toggle 0,0 through 999,999\n")));
}
