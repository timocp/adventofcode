pub struct Solver {
    min: u32,
    max: u32,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        let input: Vec<_> = input
            .trim()
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            min: input[0],
            max: input[1],
        }
    }

    fn part1(&self) -> String {
        (self.min..self.max)
            .filter(|p| valid_password(*p))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn valid_password(p: u32) -> bool {
    if p < 100000 || p > 999999 {
        // must be 6 digit number
        return false;
    }

    let mut prev_digit = p % 10;
    let mut p = p / 10;
    let mut has_double = false;

    // compare digits right to left
    while p > 0 {
        let digit = p % 10;
        // println!("checking {}{}", digit, prev_digit);

        if digit > prev_digit {
            // digits must not decrease left to right
            return false;
        }
        if digit == prev_digit {
            has_double = true;
        }

        prev_digit = digit;
        p = p / 10;
    }

    // must contain at least 1 doubled digit
    has_double
}

#[test]
fn test_valid_password() {
    assert!(valid_password(111111));
    assert!(!valid_password(223450));
    assert!(!valid_password(123789));
}
