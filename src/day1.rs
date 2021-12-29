use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => lift(&input),
            Part::Two => find_basement(&input) as i32,
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
