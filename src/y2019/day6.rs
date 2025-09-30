use std::collections::HashMap;

pub struct Solver {
    map: OrbitMap,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            map: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        count_all_orbits(&self.map.orbits).to_string()
    }

    fn part2(&self) -> String {
        count_orbital_transfers(
            &self.map.orbits,
            *self.map.objects.get("YOU").unwrap(),
            *self.map.objects.get("SAN").unwrap(),
        )
        .to_string()
    }
}

struct OrbitMap {
    // object names
    objects: HashMap<String, u32>,
    // key orbits value
    orbits: HashMap<u32, u32>,
}

fn count_all_orbits(orbits: &HashMap<u32, u32>) -> u32 {
    orbits
        .keys()
        .map(|object| count_orbits(orbits, *object))
        .sum()
}

fn count_orbits(orbits: &HashMap<u32, u32>, from: u32) -> u32 {
    if let Some(other) = orbits.get(&from) {
        count_orbits(orbits, *other) + 1
    } else {
        0
    }
}

fn count_orbital_transfers(orbits: &HashMap<u32, u32>, from: u32, to: u32) -> u32 {
    let path0 = path_to_centre(orbits, from);
    let path1 = path_to_centre(orbits, to);

    // the common part of the path doesn't need to be traversed
    let shared = shared_path_len(&path0, &path1);

    ((path0.len() - shared) + (path1.len() - shared)) as u32
}

fn path_to_centre(orbits: &HashMap<u32, u32>, mut from: u32) -> Vec<u32> {
    let mut path = vec![];

    while let Some(&parent) = orbits.get(&from) {
        path.push(parent);
        from = parent;
    }

    path.reverse();
    path
}

fn shared_path_len(path0: &[u32], path1: &[u32]) -> usize {
    for (i, (&p0, &p1)) in path0.iter().zip(path1.iter()).enumerate() {
        if p0 != p1 {
            return i;
        }
    }
    path0.len()
}

fn parse_input(input: &str) -> OrbitMap {
    let mut objects = HashMap::new();
    let mut orbits = HashMap::new();

    // COM (Center of Mass) always exists
    objects.insert("COM".to_string(), 0);

    for (i, line) in input.lines().enumerate() {
        let sep = line.find(')').unwrap();
        objects.insert(line[sep + 1..].to_string(), i as u32 + 1);
    }
    for line in input.lines() {
        let sep = line.find(')').unwrap();
        let big = *objects.get(&line[0..sep]).unwrap();
        let small = *objects.get(&line[sep + 1..]).unwrap();
        orbits.insert(small, big);
    }

    OrbitMap { objects, orbits }
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
    let map = parse_input(&TEST_INPUT);

    let objects = map.objects;
    assert_eq!(12, objects.len());
    assert_eq!(Some(&0), objects.get("COM"));
    assert_eq!(Some(&1), objects.get("B"));
    assert_eq!(Some(&10), objects.get("K"));
    assert_eq!(Some(&11), objects.get("L"));

    let orbits = map.orbits;
    assert_eq!(11, orbits.len());
    assert_eq!(objects.get("COM"), orbits.get(objects.get("B").unwrap()));
    assert_eq!(objects.get("K"), orbits.get(objects.get("L").unwrap()));
}

#[test]
fn test_count_all_orbits() {
    let map = parse_input(&TEST_INPUT);
    assert_eq!(42, count_all_orbits(&map.orbits));
}

#[cfg(test)]
const TEST_INPUT2: &str = r#"COM)B
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
K)YOU
I)SAN
"#;

#[test]
fn test_count_orbital_transfers() {
    let map = parse_input(&TEST_INPUT2);
    assert_eq!(
        4,
        count_orbital_transfers(
            &map.orbits,
            *map.objects.get("YOU").unwrap(),
            *map.objects.get("SAN").unwrap()
        )
    );
}
