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
            .filter(|p| valid_password(*p, false))
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        (self.min..self.max)
            .filter(|p| valid_password(*p, true))
            .count()
            .to_string()
    }
}

fn valid_password(p: u32, strict: bool) -> bool {
    if p < 100000 || p > 999999 {
        // must be 6 digit number
        return false;
    }

    let mut prev_digit = p % 10;
    let mut p = p / 10;

    let mut histogram = [0u32; 10];
    histogram[prev_digit as usize] = 1;

    // compare digits right to left
    while p > 0 {
        let digit = p % 10;
        // println!("checking {}{}", digit, prev_digit);

        if digit > prev_digit {
            // digits must not decrease left to right
            return false;
        }
        histogram[digit as usize] += 1;

        prev_digit = digit;
        p = p / 10;
    }

    if strict {
        // must be at least one double which is not part of a larger group
        histogram.iter().any(|c| *c == 2)
    } else {
        // must contain at least 1 doubled digit
        histogram.iter().any(|c| *c > 1)
    }
}

#[test]
fn test_valid_password() {
    assert!(valid_password(111111, false));
    assert!(valid_password(111122, false));
    assert!(valid_password(112233, false));
    assert!(valid_password(123444, false));
    assert!(valid_password(124444, false));
    assert!(!valid_password(123789, false));
    assert!(!valid_password(223450, false));
}

#[test]
fn test_valid_password_strict() {
    assert!(!valid_password(111111, true));
    assert!(valid_password(111122, true));
    assert!(valid_password(112233, true));
    assert!(!valid_password(123444, true));
    assert!(!valid_password(124444, true));
    assert!(!valid_password(123789, true));
    assert!(!valid_password(223450, true));
}
