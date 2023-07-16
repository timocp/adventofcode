use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let weights = parse_input(input);
    format!(
        "{}",
        find_group1(
            &weights,
            match part {
                Part::One => 3,
                Part::Two => 4,
            },
        )
        .quantum_entanglement(&weights)
    )
}

// A bitmap representing a group of packages
// bit 0 = 0th package, bit 1 = 1st package, etc.
// This only works as long as there are at least as many bits as packages
#[derive(Debug, Clone)]
struct Group(u64);

impl Group {
    fn new(value: u64) -> Self {
        Group(value)
    }

    fn package_count(&self) -> u32 {
        self.0.count_ones()
    }

    fn weights(&self, weights: &[u32]) -> Vec<u32> {
        let mut result = vec![];
        for (i, weight) in weights.iter().enumerate() {
            if (self.0 & (1 << i)) != 0 {
                result.push(*weight);
            }
        }
        result
    }

    fn quantum_entanglement(&self, weights: &[u32]) -> u64 {
        self.weights(weights)
            .iter()
            .fold(1, |acc, w| acc * (*w as u64))
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>()
}

// group1 is the group of minimum size that has the smallest quantum entanglement
fn find_group1(weights: &[u32], count: u32) -> Group {
    let group_weight = weights.iter().sum::<u32>() / count;

    let groups = find_groups(weights, group_weight);

    let group1_size = groups.iter().map(|g| g.package_count()).min().unwrap();

    groups
        .iter()
        .filter(|g| g.package_count() == group1_size)
        .map(|g| (g, g.quantum_entanglement(weights)))
        .min_by(|(_, qe1), (_, qe2)| qe1.cmp(qe2))
        .unwrap()
        .0
        .clone()
}

fn find_groups(weights: &[u32], group_weight: u32) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];

    for (i, weight) in weights.iter().enumerate() {
        if *weight == group_weight {
            groups.push(Group::new(1 << i));
        } else if *weight < group_weight && i + 1 < weights.len() {
            for subgroup in find_groups(&weights[(i + 1)..], group_weight - weight).iter() {
                groups.push(Group::new(subgroup.0 << (i + 1) | (1 << i)));
            }
        }
    }

    groups
}

#[test]
fn test() {
    let weights = vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11];

    let groups = find_groups(&weights, 20);
    assert_eq!(25, groups.len());
    for group in groups {
        assert_eq!(20, group.weights(&weights).iter().sum::<u32>());
    }

    let group = find_group1(&weights, 3);
    assert_eq!(vec![9, 11], group.weights(&weights));
    assert_eq!(99, group.quantum_entanglement(&weights));

    let groups = find_groups(&weights, 15);
    assert_eq!(16, groups.len());
    for group in groups {
        assert_eq!(15, group.weights(&weights).iter().sum::<u32>());
    }

    let group = find_group1(&weights, 4);
    assert_eq!(vec![4, 11], group.weights(&weights));
    assert_eq!(44, group.quantum_entanglement(&weights));
}
