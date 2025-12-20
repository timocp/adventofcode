pub fn parse_input(input: &str) -> Vec<Problem> {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut problems: Vec<Problem> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(Operation::from)
        .map(|operation| Problem {
            operation,
            numbers: vec![],
        })
        .collect();

    for line in lines {
        for (i, n) in line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .enumerate()
        {
            problems[i].numbers.push(n);
        }
    }

    problems
}

#[derive(Debug)]
pub struct Problem {
    operation: Operation,
    numbers: Vec<u64>,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Mul,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        if s == "+" {
            Operation::Add
        } else if s == "*" {
            Operation::Mul
        } else {
            panic!()
        }
    }
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.numbers.iter().fold(0, |acc, e| acc + *e),
            Operation::Mul => self.numbers.iter().fold(1, |acc, e| acc * *e),
        }
    }
}

pub fn part1(input: &[Problem]) -> u64 {
    input.iter().map(|p| p.solve()).sum()
}

pub fn part2(input: &[Problem]) -> &str {
    "unimplemented"
}

#[test]
fn test() {
    let test_input = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
    let input = parse_input(test_input);
    assert_eq!(4277556, part1(&input));
}
