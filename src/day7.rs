use crate::Part;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let wires = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&wires),
            Part::Two => 0,
        }
    )
}

fn part1(wires: &HashMap<String, Wire>) -> u16 {
    measure(wires, "a", &mut HashMap::new())
}

fn measure(wires: &HashMap<String, Wire>, target: &str, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(v) = cache.get(target) {
        return *v;
    }

    let signal = if let Some(wire) = wires.get(target) {
        match wire {
            Wire::Signal(v) => *v,
            Wire::Direct(a) => measure(wires, a, cache),
            Wire::And(a, b) => measure(wires, a, cache) & measure(wires, b, cache),
            Wire::Or(a, b) => measure(wires, a, cache) | measure(wires, b, cache),
            Wire::Lshift(a, v) => measure(wires, a, cache) << v,
            Wire::Rshift(a, v) => measure(wires, a, cache) >> v,
            Wire::Not(a) => !measure(wires, a, cache),
        }
    } else {
        // immediate value instead of a wire name
        target.parse::<u16>().unwrap()
    };
    cache.insert(target.to_owned(), signal);
    signal
}

#[derive(Clone, Debug)]
enum Wire {
    Signal(u16),
    Direct(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

fn parse_input(input: &str) -> HashMap<String, Wire> {
    let mut map = HashMap::new();
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^(\d+) -> (\w+)$").unwrap();
        static ref RE2: Regex = Regex::new(r"^(\w+) (AND|OR) (\w+) -> (\w+)$").unwrap();
        static ref RE3: Regex = Regex::new(r"^(\w+) (LSHIFT|RSHIFT) (\d+) -> (\w+)$").unwrap();
        static ref RE4: Regex = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
        static ref RE5: Regex = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
    }
    for line in input.lines() {
        if let Some(caps) = RE1.captures(line) {
            map.insert(caps[2].to_owned(), Wire::Signal(caps[1].parse().unwrap()));
        } else if let Some(caps) = RE2.captures(line) {
            map.insert(
                caps[4].to_owned(),
                match &caps[2] {
                    "AND" => Wire::And(caps[1].to_owned(), caps[3].to_owned()),
                    "OR" => Wire::Or(caps[1].to_owned(), caps[3].to_owned()),
                    _ => unreachable!(),
                },
            );
        } else if let Some(caps) = RE3.captures(line) {
            map.insert(
                caps[4].to_owned(),
                match &caps[2] {
                    "LSHIFT" => Wire::Lshift(caps[1].to_owned(), caps[3].parse().unwrap()),
                    "RSHIFT" => Wire::Rshift(caps[1].to_owned(), caps[3].parse().unwrap()),
                    _ => unreachable!(),
                },
            );
        } else if let Some(caps) = RE4.captures(line) {
            map.insert(caps[2].to_owned(), Wire::Not(caps[1].to_owned()));
        } else if let Some(caps) = RE5.captures(line) {
            map.insert(caps[2].to_owned(), Wire::Direct(caps[1].to_owned()));
        } else {
            panic!("unexpected instruction: {}", line);
        }
    }

    map
}

#[test]
fn test() {
    let test_input = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i
";
    let wires = parse_input(test_input);
    dbg!(&wires);
    assert_eq!(72, measure(&wires, "d", &mut HashMap::new()));
    assert_eq!(507, measure(&wires, "e", &mut HashMap::new()));
    assert_eq!(492, measure(&wires, "f", &mut HashMap::new()));
    assert_eq!(114, measure(&wires, "g", &mut HashMap::new()));
    assert_eq!(65412, measure(&wires, "h", &mut HashMap::new()));
    assert_eq!(65079, measure(&wires, "i", &mut HashMap::new()));
    assert_eq!(123, measure(&wires, "x", &mut HashMap::new()));
    assert_eq!(456, measure(&wires, "y", &mut HashMap::new()));
}
