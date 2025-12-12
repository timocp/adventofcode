pub struct Solver {
    banks: Vec<Vec<u64>>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            banks: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        sum_best_joltage(&self.banks, 2).to_string()
    }

    fn part2(&self) -> String {
        sum_best_joltage(&self.banks, 12).to_string()
    }
}

fn sum_best_joltage(banks: &[Vec<u64>], digits: usize) -> u64 {
    banks.iter().map(|bank| best_joltage(bank, digits)).sum()
}

fn best_joltage(bank: &[u64], digits: usize) -> u64 {
    let mut best = 0;
    let mut best_at = 0;
    for (i, battery) in bank.iter().enumerate().take(bank.len() - digits + 1) {
        if *battery > best {
            best = *battery;
            best_at = i;
        }
    }
    if digits > 1 {
        best * 10u64.pow((digits - 1).try_into().unwrap())
            + best_joltage(&bank[(best_at + 1)..], digits - 1)
    } else {
        best
    }
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as u64 - 48).collect())
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
    let banks = parse_input(test_input);

    assert_eq!(98, best_joltage(&banks[0], 2));
    assert_eq!(89, best_joltage(&banks[1], 2));
    assert_eq!(78, best_joltage(&banks[2], 2));
    assert_eq!(92, best_joltage(&banks[3], 2));
    assert_eq!(357, sum_best_joltage(&banks, 2));

    assert_eq!(987654321111, best_joltage(&banks[0], 12));
    assert_eq!(811111111119, best_joltage(&banks[1], 12));
    assert_eq!(434234234278, best_joltage(&banks[2], 12));
    assert_eq!(888911112111, best_joltage(&banks[3], 12));
    assert_eq!(3121910778619, sum_best_joltage(&banks, 12));
}
