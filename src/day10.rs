use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => expand(&input, 40),
            Part::Two => expand(&input, 50),
        }
    )
}

fn expand(num: &[u8], times: usize) -> usize {
    let mut num: Vec<u8> = num.iter().copied().collect();
    for _ in 0..times {
        num = step(&num);
    }
    num.len()
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect()
}

fn step(num: &[u8]) -> Vec<u8> {
    let mut out = vec![];
    let mut last = num[0];
    let mut count = 1;
    for &n in num.iter().skip(1) {
        if n == last {
            count += 1;
        } else {
            out.push(count);
            out.push(last);
            count = 1;
            last = n;
        }
    }
    out.push(count);
    out.push(last);
    out
}

#[test]
fn test() {
    assert_eq!(vec![1, 1], step(&[1]));
    assert_eq!(vec![2, 1], step(&[1, 1]));
    assert_eq!(vec![1, 2, 1, 1], step(&[2, 1]));
    assert_eq!(vec![1, 1, 1, 2, 2, 1], step(&[1, 2, 1, 1]));
    assert_eq!(vec![3, 1, 2, 2, 1, 1], step(&[1, 1, 1, 2, 2, 1]));
    assert_eq!(
        vec![1, 3, 2, 1, 2, 3, 2, 2, 2, 1, 1, 3],
        step(&[3, 1, 1, 3, 3, 2, 2, 1, 1, 3])
    );
}
