use std::collections::HashMap;

use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    let result = combinations(&input, 150);
    format!(
        "{}",
        match part {
            Part::One => result.total_combinations(),
            Part::Two => result.min_combinations(),
        }
    )
}

struct Stats(HashMap<u32, u32>);

impl Stats {
    fn new() -> Self {
        Stats(HashMap::new())
    }

    fn increment(&mut self, key: u32) {
        self.0.entry(key).and_modify(|e| *e += 1).or_insert(1);
    }

    fn total_combinations(&self) -> u32 {
        self.0.values().sum()
    }

    fn min_containers(&self) -> u32 {
        match self.0.keys().min() {
            Some(key) => *key,
            None => 0,
        }
    }

    fn min_combinations(&self) -> u32 {
        match self.0.get(&self.min_containers()) {
            Some(value) => *value,
            None => 0,
        }
    }
}

// Returns a HashMap of <number of containers, number of combinations>
fn combinations(input: &Vec<i32>, target: i32) -> Stats {
    let mut stats = Stats::new();
    search_combinations(input, target, &mut stats, 0, 0);
    stats
}

fn search_combinations(input: &Vec<i32>, target: i32, stats: &mut Stats, count: u32, index: usize) {
    if target == 0 {
        stats.increment(count);
        return;
    }
    if target < 0 {
        return;
    }
    if index == input.len() {
        return;
    }
    // search rest of vector including this container
    search_combinations(input, target - input[index], stats, count + 1, index + 1);
    // search rest of vector without including this container
    search_combinations(input, target, stats, count, index + 1);
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[test]
fn test() {
    let result = combinations(&parse_input("20\n15\n10\n5\n5"), 25);
    assert_eq!(result.total_combinations(), 4);
    assert_eq!(result.min_containers(), 2);
    assert_eq!(result.min_combinations(), 3)
}
