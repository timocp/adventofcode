use crate::pos::Pos;

pub fn parse_input(input: &str) -> Vec<Pos> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[Pos]) -> u64 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input.iter().skip(i + 1).map(|b| area(a, b)))
        .max()
        .unwrap()
}

fn area(a: &Pos, b: &Pos) -> u64 {
    ((a.x - b.x).unsigned_abs() as u64 + 1) * ((a.y - b.y).unsigned_abs() as u64 + 1)
}

pub fn part2(input: &[Pos]) -> &str {
    "unimplemented"
}

#[test]
fn test() {
    let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    let input = parse_input(test_input);
    assert_eq!(50, part1(&input));
}
