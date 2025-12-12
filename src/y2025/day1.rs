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
        count_zeros(&self.input, false).to_string()
    }

    fn part2(&self) -> String {
        count_zeros(&self.input, true).to_string()
    }
}

fn count_zeros(turns: &[i32], all: bool) -> i32 {
    let mut pos = 50;
    let mut count = 0;
    for turn in turns {
        let started_at_zero = pos == 0;
        pos += turn;

        let q = pos / 100;
        if all {
            match (started_at_zero, pos) {
                (true, ..0) => count += -q,
                (false, ..=0) => count += -q + 1,
                (_, 100..) => count += q,
                (_, _) => {}
            }
        } else if pos % 100 == 0 {
            count += 1;
        }

        pos = pos.rem_euclid(100);
    }
    count
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            if let Some(i) = line.strip_prefix("L") {
                -i.parse::<i32>().unwrap()
            } else if let Some(i) = line.strip_prefix("R") {
                i.parse().unwrap()
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
    assert_eq!(3, count_zeros(&turns, false));
    assert_eq!(6, count_zeros(&turns, true));

    assert_eq!(10, count_zeros(&parse_input("R1000\n"), true));
    assert_eq!(10, count_zeros(&parse_input("L1000\n"), true));
    assert_eq!(1, count_zeros(&parse_input("L50\nR1\n"), true));
    assert_eq!(1, count_zeros(&parse_input("R50\nL1\n"), true));
}
