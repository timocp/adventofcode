use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let key = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&key),
            Part::Two => 0,
        }
    )
}

fn parse_input(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

fn part1(key: &str) -> usize {
    let mut num: usize = 0;
    loop {
        let digest = md5::compute(format!("{}{}", key, num).as_bytes());
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            return num;
        }
        num += 1;
    }
}

#[test]
fn test() {
    assert_eq!(609043, part1(&parse_input("abcdef\n")));
    assert_eq!(1048970, part1(&parse_input("pqrstuv\n")));
}
