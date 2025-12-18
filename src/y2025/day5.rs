#[derive(Debug)]
pub struct Input {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

pub fn parse_input(input: &str) -> Input {
    let mut fresh_ranges = vec![];
    let mut ingredients = vec![];

    for line in input.lines().filter(|&line| line.len() > 0) {
        if let Some(hyphen) = line.find(|c| c == '-') {
            fresh_ranges.push((
                line[0..hyphen].parse().unwrap(),
                line[(hyphen + 1)..].parse().unwrap(),
            ));
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    Input {
        fresh_ranges,
        ingredients,
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|&&ingredient| {
            input
                .fresh_ranges
                .iter()
                .any(|range| ingredient >= range.0 && ingredient <= range.1)
        })
        .count()
}

pub fn part2(input: &Input) -> &str {
    "unimplemented"
}

#[test]
fn test() {
    let test_input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    let input = parse_input(test_input);
    assert_eq!(3, part1(&input));
}
