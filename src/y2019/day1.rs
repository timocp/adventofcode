pub struct Solver {
    module_masses: Vec<i32>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            module_masses: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        sum_fuel(&self.module_masses, fuel_required).to_string()
    }

    fn part2(&self) -> String {
        sum_fuel(&self.module_masses, real_fuel_required).to_string()
    }
}

fn sum_fuel(masses: &[i32], f: impl Fn(i32) -> i32) -> i32 {
    masses.iter().map(|&m| f(m)).sum()
}

fn fuel_required(mass: i32) -> i32 {
    let f = mass / 3 - 2;
    if f >= 0 { f } else { 0 }
}

fn real_fuel_required(mass: i32) -> i32 {
    let f = fuel_required(mass);
    f + if f > 0 { real_fuel_required(f) } else { 0 }
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[test]
fn test_fuel_required() {
    assert_eq!(2, fuel_required(12));
    assert_eq!(2, fuel_required(14));
    assert_eq!(654, fuel_required(1969));
    assert_eq!(33583, fuel_required(100756));
}

#[test]
fn test_real_fuel_required() {
    assert_eq!(2, real_fuel_required(14));
    assert_eq!(966, real_fuel_required(1969));
    assert_eq!(50346, real_fuel_required(100756));
}
