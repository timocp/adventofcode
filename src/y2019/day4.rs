pub struct Input {
    min: u32,
    max: u32,
}

pub fn parse_input(input: &str) -> Input {
    let input: Vec<_> = input
        .trim()
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();
    Input {
        min: input[0],
        max: input[1],
    }
}

pub fn part1(input: &Input) -> usize {
    (input.min..input.max)
        .filter(|p| valid_password(*p, false))
        .count()
}

pub fn part2(input: &Input) -> usize {
    (input.min..input.max)
        .filter(|p| valid_password(*p, true))
        .count()
}

fn valid_password(p: u32, strict: bool) -> bool {
    if !(100000..=999999).contains(&p) {
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
        p /= 10;
    }

    if strict {
        // must be at least one double which is not part of a larger group
        histogram.contains(&2)
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
