pub struct Solver {
    input: Vec<u8>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        expand(&self.input, 40).to_string()
    }

    fn part2(&self) -> String {
        expand(&self.input, 50).to_string()
    }
}

fn expand(num: &[u8], times: usize) -> usize {
    let mut num: Vec<u8> = num.to_vec();
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
