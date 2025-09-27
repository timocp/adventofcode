pub struct Solver {
    module_masses: Vec<u32>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            module_masses: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        self.module_masses
            .iter()
            .map(|&m| fuel_required(m))
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn fuel_required(mass: u32) -> u32 {
    mass / 3 - 2
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[test]
fn test_fuel_required() {
    assert_eq!(2, fuel_required(12));
    assert_eq!(2, fuel_required(14));
    assert_eq!(654, fuel_required(1969));
    assert_eq!(33583, fuel_required(100756));
}
