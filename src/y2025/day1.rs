pub struct Solver {
    input: Vec<Turn>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        count_zeros(&self.input).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

#[derive(Debug)]
enum Turn {
    Left(i32),
    Right(i32),
}

fn count_zeros(turns: &[Turn]) -> usize {
    let mut pos = 50;
    let mut count = 0;
    for turn in turns {
        match turn {
            Turn::Left(i) => pos -= i,
            Turn::Right(i) => pos += i,
        }
        pos %= 100;
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Turn> {
    input
        .lines()
        .map(|line| {
            if let Some(i) = line.strip_prefix("L") {
                Turn::Left(i.parse().unwrap())
            } else if let Some(i) = line.strip_prefix("R") {
                Turn::Right(i.parse().unwrap())
            } else {
                panic!()
            }
        })
        .collect()
}

#[test]
fn test() {
    let test_input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    let turns = parse_input(test_input);
    assert_eq!(3, count_zeros(&turns));
}
