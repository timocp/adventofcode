// split into columns.  the operator in the last line indicates the start of each column
pub fn parse_input(input: &str) -> Vec<Problem<'_>> {
    let mut lines: Vec<&str> = input.lines().collect();

    let op_line = lines.pop().unwrap();

    let mut indexes: Vec<_> = op_line
        .char_indices()
        .filter(|(_i, c)| !c.is_whitespace())
        .collect();
    indexes.push((op_line.len() + 1, '\n'));

    indexes
        .windows(2)
        .map(|col| Problem {
            operation: Operation::from(col[0].1),
            column: lines
                .iter()
                .map(|line| &line[col[0].0..(col[1].0 - 1)])
                .collect(),
        })
        .collect()
}

#[derive(Debug)]
pub struct Problem<'a> {
    operation: Operation,
    column: Vec<&'a str>,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Mul,
}

impl From<char> for Operation {
    fn from(c: char) -> Self {
        if c == '+' {
            Operation::Add
        } else if c == '*' {
            Operation::Mul
        } else {
            panic!()
        }
    }
}

impl Problem<'_> {
    fn solve1(&self) -> u64 {
        let numbers: Vec<u64> = self
            .column
            .iter()
            .map(|s| s.trim().parse().unwrap())
            .collect();
        match self.operation {
            Operation::Add => numbers.iter().fold(0, |acc, e| acc + *e),
            Operation::Mul => numbers.iter().fold(1, |acc, e| acc * *e),
        }
    }

    // for part 2, the digits are in columns
    fn solve2(&self) -> u64 {
        let numbers: Vec<u64> = (0..self.column[0].len())
            .map(|i| {
                let mut n = 0;
                for row in &self.column {
                    let digit = row.chars().nth(i).unwrap();
                    if !digit.is_whitespace() {
                        n *= 10;
                        n += digit.to_digit(10).unwrap();
                    }
                }
                n as u64
            })
            .collect();
        match self.operation {
            Operation::Add => numbers.iter().fold(0, |acc, e| acc + *e),
            Operation::Mul => numbers.iter().fold(1, |acc, e| acc * *e),
        }
    }
}

pub fn part1(input: &[Problem]) -> u64 {
    input.iter().map(|p| p.solve1()).sum()
}

pub fn part2(input: &[Problem]) -> u64 {
    input.iter().map(|p| p.solve2()).sum()
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
    assert_eq!(3263827, part2(&input));
}
