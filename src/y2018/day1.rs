use std::collections::HashSet;

pub struct Solver {
    input: Vec<i32>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        sum_lines(&self.input).to_string()
    }

    fn part2(&self) -> String {
        first_reached_twice(&self.input).to_string()
    }
}

fn sum_lines(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn first_reached_twice(input: &[i32]) -> i32 {
    let mut sum = 0;
    let mut seen = HashSet::new();
    seen.insert(sum);

    for i in input.iter().cycle() {
        sum += i;
        if seen.contains(&sum) {
            return sum;
        }
        seen.insert(sum);
    }
    unreachable!();
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect()
}

#[test]
fn test_sum_lines() {
    assert_eq!(3, sum_lines(&vec![1, 1, 1]));
    assert_eq!(0, sum_lines(&vec![1, 1, -2]));
    assert_eq!(-6, sum_lines(&vec![-1, -2, -3]));
}

#[test]
fn test_first_reached_twice() {
    assert_eq!(0, first_reached_twice(&vec![1, -1]));
    assert_eq!(10, first_reached_twice(&vec![3, 3, 4, -2, -4]));
    assert_eq!(5, first_reached_twice(&vec![-6, 3, 8, 5, -6]));
    assert_eq!(14, first_reached_twice(&vec![7, 7, -2, -7, -4]));
}
