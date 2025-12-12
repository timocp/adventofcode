pub struct Solver {
    input: Vec<Vec<u32>>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        sum_best_joltage(&self.input).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn sum_best_joltage(banks: &[Vec<u32>]) -> u32 {
    banks.iter().map(|batteries| best_joltage(batteries)).sum()
}

fn best_joltage(bank: &[u32]) -> u32 {
    for d1 in (1..=9).rev() {
        if let Some(pos) = bank.iter().position(|&n| n == d1) {
            // d1 is best first digit. what is the best next digit to its right?
            if let Some(d2) = max_in(&bank[(pos + 1)..]) {
                return d1 * 10 + d2;
            }
        }
    }
    panic!("no solution in {:?}", bank);
}

fn max_in(batteries: &[u32]) -> Option<u32> {
    let mut max: Option<u32> = None;
    for d2 in batteries {
        max = Some(match max {
            None => *d2,
            Some(i) => i.max(*d2),
        });
    }
    max
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u32 - 48).collect())
        .collect()
}

#[test]
fn test() {
    let test_input = "\
987654321111111
811111111111119
234234234234278
818181911112111
";
    let batteries = parse_input(test_input);

    assert_eq!(98, best_joltage(&batteries[0]));
    assert_eq!(89, best_joltage(&batteries[1]));
    assert_eq!(78, best_joltage(&batteries[2]));
    assert_eq!(92, best_joltage(&batteries[3]));
    assert_eq!(17346, sum_best_joltage(&batteries));
}
