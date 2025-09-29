use std::collections::HashMap;

pub struct Solver {
    // key orbits value
    orbits: HashMap<String, String>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            orbits: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        count_all_orbits(&self.orbits).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn count_all_orbits(orbits: &HashMap<String, String>) -> u32 {
    orbits
        .keys()
        .map(|object| count_orbits(orbits, object))
        .sum()
}

fn count_orbits(orbits: &HashMap<String, String>, from: &str) -> u32 {
    if let Some(other) = orbits.get(from) {
        count_orbits(orbits, other) + 1
    } else {
        0
    }
}

fn parse_input(input: &str) -> HashMap<String, String> {
    let mut hash = HashMap::new();
    for line in input.lines() {
        let sep = line.find(')').unwrap();
        hash.insert(line[sep + 1..].to_string(), line[0..sep].to_string());
    }
    hash
}

#[cfg(test)]
const TEST_INPUT: &str = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
"#;

#[test]
fn test_parse_input() {
    let orbits = parse_input(&TEST_INPUT);

    assert_eq!(11, orbits.len());
    assert_eq!("C", orbits.get("D").unwrap());
    assert_eq!("B", orbits.get("C").unwrap());
    assert_eq!("COM", orbits.get("B").unwrap());
    assert_eq!(None, orbits.get("COM"));
}

#[test]
fn test_count_all_orbits() {
    let orbits = parse_input(&TEST_INPUT);
    assert_eq!(42, count_all_orbits(&orbits));
}
