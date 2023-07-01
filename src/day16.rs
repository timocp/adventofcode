use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&input).unwrap(),
            Part::Two => part2(&input).unwrap(),
        }
    )
}

fn part1(input: &Vec<Aunt>) -> Option<u32> {
    for aunt in input {
        if aunt.items.iter().all(|(item, value)| match item.as_str() {
            "children" => *value == 3,
            "cats" => *value == 7,
            "samoyeds" => *value == 2,
            "pomeranians" => *value == 3,
            "akitas" => *value == 0,
            "vizslas" => *value == 0,
            "goldfish" => *value == 5,
            "trees" => *value == 3,
            "cars" => *value == 2,
            "perfumes" => *value == 1,
            _ => panic!("unknown item: {}", item),
        }) {
            return Some(aunt.number);
        }
    }
    None
}

fn part2(input: &Vec<Aunt>) -> Option<u32> {
    for aunt in input {
        if aunt.items.iter().all(|(item, value)| match item.as_str() {
            "children" => *value == 3,
            "cats" => *value > 7,
            "samoyeds" => *value == 2,
            "pomeranians" => *value < 3,
            "akitas" => *value == 0,
            "vizslas" => *value == 0,
            "goldfish" => *value < 5,
            "trees" => *value > 3,
            "cars" => *value == 2,
            "perfumes" => *value == 1,
            _ => panic!("unknown item: {}", item),
        }) {
            return Some(aunt.number);
        }
    }
    None
}

#[derive(Debug)]
struct Aunt {
    number: u32,
    items: Vec<(String, u32)>,
}

// example input:
// Sue 1: children: 1, cars: 8, vizslas: 7
// Sue 2: akitas: 10, perfumes: 10, children: 5
fn parse_input(input: &str) -> Vec<Aunt> {
    let trailing_chars = [',', ':'];
    input
        .lines()
        .map(|line| {
            let mut number = 0;
            let mut items = vec![];
            let mut words = line.split(' ');
            while let Some(word) = words.next() {
                let value: u32 = words
                    .next()
                    .unwrap()
                    .trim_end_matches(&trailing_chars)
                    .parse()
                    .unwrap();
                match word.trim_end_matches(&trailing_chars) {
                    "Sue" => number = value,
                    _ => items.push((word.trim_end_matches(&trailing_chars).to_string(), value)),
                }
            }
            Aunt { number, items }
        })
        .collect()
}

#[test]
fn test() {
    // assert_eq!()
}
