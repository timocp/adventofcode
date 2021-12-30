use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let key = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&key),
            Part::Two => part2(&key),
        }
    )
}

fn parse_input(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

fn search_hash(key: &str, test: impl Fn(md5::Digest) -> bool) -> usize {
    let mut num = 0;
    loop {
        let digest = md5::compute(format!("{}{}", key, num).as_bytes());
        if test(digest) {
            return num;
        }
        num += 1;
    }
}

fn part1(key: &str) -> usize {
    search_hash(key, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] < 16
    })
}

fn part2(key: &str) -> usize {
    search_hash(key, |digest| {
        digest[0] == 0 && digest[1] == 0 && digest[2] == 0
    })
}

#[test]
fn test() {
    assert_eq!(609043, part1(&parse_input("abcdef\n")));
    assert_eq!(1048970, part1(&parse_input("pqrstuv\n")));
}
