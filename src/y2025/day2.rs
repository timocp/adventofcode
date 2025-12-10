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
        sum_invalid(&self.input).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
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

fn sum_invalid(ranges: &[(u64, u64)]) -> u64 {
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
    assert_eq!(1227775554, sum_invalid(&parse_input(test_input)));
}
