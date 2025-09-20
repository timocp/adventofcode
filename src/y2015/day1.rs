pub struct Solver {
    directions: Vec<char>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            directions: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        lift(&self.directions).to_string()
    }

    fn part2(&self) -> String {
        find_basement(&self.directions).to_string()
    }
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn lift(directions: &[char]) -> i32 {
    directions
        .iter()
        .fold(0, |acc, &c| acc + if c == '(' { 1 } else { -1 })
}

fn find_basement(directions: &[char]) -> usize {
    let mut level = 0;
    for (i, &d) in directions.iter().enumerate() {
        level += if d == '(' { 1 } else { -1 };
        if level == -1 {
            return i + 1;
        }
    }
    panic!("never reached basement");
}

#[test]
fn test() {
    assert_eq!(0, lift(&parse_input("(())\n")));
    assert_eq!(0, lift(&parse_input("()()\n")));
    assert_eq!(3, lift(&parse_input("(((\n")));
    assert_eq!(3, lift(&parse_input("(()(()(\n")));
    assert_eq!(3, lift(&parse_input("))(((((\n")));
    assert_eq!(-1, lift(&parse_input("())\n")));
    assert_eq!(-1, lift(&parse_input("))(\n")));
    assert_eq!(-3, lift(&parse_input(")))\n")));
    assert_eq!(-3, lift(&parse_input(")())())\n")));

    assert_eq!(1, find_basement(&parse_input(")\n")));
    assert_eq!(5, find_basement(&parse_input("()())\n")));
}
