use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => lift(&input),
            Part::Two => 0,
        }
    )
}

fn parse_input(input: &str) -> Vec<char> {
    input.lines().next().unwrap().chars().collect()
}

fn lift(directions: &[char]) -> i32 {
    directions
        .iter()
        .fold(0, |acc, &c| acc + if c == '(' { 1 } else { -1 })
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
}
