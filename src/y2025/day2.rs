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
    let len = n.ilog10() + 1;
    if len.is_multiple_of(2) {
        let divisor = 10u64.pow(len / 2);
        n % divisor == n / divisor
    } else {
        false
    }
}

// max in input is 10 digits, but higher than max u32
#[allow(clippy::zero_prefixed_literal, reason = "describes repeat pattern")]
fn invalid2(n: u64) -> bool {
    // a repeating digit will be a multiple
    // eg if 6 digits, will be AAAAAA or ABABAB or ABCABC
    // -> divisible by 111111 or 010101 or 001001
    match n {
        0..=9 => false,
        10..=99 => n.is_multiple_of(11),
        100..=999 => n.is_multiple_of(111),
        1000..=9999 => n.is_multiple_of(1111) || n.is_multiple_of(0101),
        10000..=99999 => n.is_multiple_of(11111),
        100000..=999999 => {
            n.is_multiple_of(111111) || n.is_multiple_of(010101) || n.is_multiple_of(001001)
        }
        1000000..=9999999 => n.is_multiple_of(1111111),
        10000000..=99999999 => {
            n.is_multiple_of(11111111) || n.is_multiple_of(01010101) || n.is_multiple_of(00010001)
        }
        100000000..=999999999 => n.is_multiple_of(111111111) || n.is_multiple_of(001001001),
        1000000000..=9999999999 => {
            n.is_multiple_of(1111111111)
                || n.is_multiple_of(0101010101)
                || n.is_multiple_of(0000100001)
        }
        _ => panic!(),
    }
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
