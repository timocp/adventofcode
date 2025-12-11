pub struct Solver {
    input: Vec<(u64, u64)>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        sum_invalid(&self.input, invalid).to_string()
    }

    fn part2(&self) -> String {
        sum_invalid(&self.input, invalid2).to_string()
    }
}

fn invalid(n: u64) -> bool {
    let s = format!("{}", n);
    if s.len() % 2 == 0 {
        let (front, back) = s.split_at(s.len() / 2);
        front == back
    } else {
        false
    }
}

// max in input is 10 digits
fn invalid2(n: u64) -> bool {
    match n {
        0..=9 => false,
        10..=99 => check(n, 1, 2),
        100..=999 => check(n, 1, 3),
        1000..=9999 => check(n, 1, 4) || check(n, 2, 2),
        10000..=99999 => check(n, 1, 5),
        100000..=999999 => check(n, 1, 6) || check(n, 2, 3) || check(n, 3, 2),
        1000000..=9999999 => check(n, 1, 7),
        10000000..=99999999 => check(n, 1, 8) || check(n, 2, 4) || check(n, 4, 2),
        100000000..=999999999 => check(n, 1, 9) || check(n, 3, 3),
        1000000000..=9999999999 => check(n, 1, 10) || check(n, 2, 5) || check(n, 5, 2),
        _ => panic!(),
    }
}

// check if n in decimal is made up of R repeats of D digits
fn check(mut n: u64, digits: u32, repeats: u32) -> bool {
    let divisor = 10u32.pow(digits) as u64;
    let pat = n % divisor;
    for _ in 1..repeats {
        n /= divisor;
        if pat != n % divisor {
            return false;
        }
    }
    true
}

fn sum_invalid(ranges: &[(u64, u64)], invalid: fn(u64) -> bool) -> u64 {
    let mut sum = 0;

    for range in ranges {
        for n in range.0..=range.1 {
            if invalid(n) {
                sum += n;
            }
        }
    }

    sum
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect::<Vec<_>>())
        .map(|range| (range[0].parse().unwrap(), range[1].parse().unwrap()))
        .collect()
}

#[test]
fn test() {
    let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    assert_eq!(1227775554, sum_invalid(&parse_input(test_input), invalid));
    assert_eq!(4174379265, sum_invalid(&parse_input(test_input), invalid2));
}
