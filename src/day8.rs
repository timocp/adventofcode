use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input),
        }
    )
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|s| s.as_bytes()).collect()
}

fn part1(input: &[&[u8]]) -> usize {
    input
        .iter()
        .map(|line| line.len() - decode(line).len())
        .sum()
}

fn part2(input: &[&[u8]]) -> usize {
    input
        .iter()
        .map(|line| encode(line).len() - line.len())
        .sum()
}

fn hexdigit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - (b'a' - 10),
        b'A'..=b'A' => c - (b'A' - 10),
        _ => panic!("non-hex character in hex escape: {}", c),
    }
}

// Cannot use rust strings due to UTF8 encoding; puzzle input includes escapes which would be 2
// byte UTF characters but they want 1 byte for the answer.  Use byte slices instead.
fn decode(s: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = vec![];
    let mut i = 1;

    // check for surrounding quotes
    assert_eq!(b'"', s[0]);
    assert_eq!(b'"', s[s.len() - 1]);

    // println!("s: {}", s.iter().map(|&u| u as char).collect::<String>());
    while i < s.len() - 1 {
        if s[i] == b'\\' {
            if s[i + 1] == b'x' {
                o.push(hexdigit(s[i + 2]) * 16 + hexdigit(s[i + 3]));
                i += 4
            } else {
                o.push(s[i + 1]);
                i += 2
            }
        } else {
            o.push(s[i]);
            i += 1;
        }
    }
    // println!("o: {}", o.iter().map(|&u| u as char).collect::<String>());
    o
}

fn encode(s: &[u8]) -> Vec<u8> {
    let mut o: Vec<u8> = vec![b'\"'];

    // println!("s: {}", s.iter().map(|&u| u as char).collect::<String>());
    for b in s {
        match b {
            b'\\' | b'"' => {
                o.push(b'\\');
                o.push(*b);
            }
            b' '..=b'~' => o.push(*b),
            _ => panic!("unable to encode: {}", b), // could escape with \x00 but input is printable ascii only
        }
    }
    o.push(b'\"');
    // println!("o: {}", o.iter().map(|&u| u as char).collect::<String>());
    o
}

#[test]
fn test() {
    let test1 = r#""""#.as_bytes();
    let test2 = r#""abc""#.as_bytes();
    let test3 = r#""aaa\"aaa""#.as_bytes();
    let test4 = r#""\x27""#.as_bytes();
    assert_eq!(2, test1.len());
    assert_eq!(0, decode(test1).len());
    assert_eq!(6, encode(test1).len());
    assert_eq!(5, test2.len());
    assert_eq!(3, decode(test2).len());
    assert_eq!(9, encode(test2).len());
    assert_eq!(10, test3.len());
    assert_eq!(7, decode(test3).len());
    assert_eq!(16, encode(test3).len());
    assert_eq!(6, test4.len());
    assert_eq!(1, decode(test4).len());
    assert_eq!(11, encode(test4).len());
    assert_eq!(12, part1(&vec![test1, test2, test3, test4]));
    assert_eq!(19, part2(&vec![test1, test2, test3, test4]));
}
