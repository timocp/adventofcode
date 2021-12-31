use crate::Part;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let minmax = parse_input(input).minmax_distance();
    format!(
        "{}",
        match part {
            Part::One => minmax.0,
            Part::Two => minmax.1,
        }
    )
}

struct Graph<'a> {
    names: Vec<&'a str>,
    distance: HashMap<(usize, usize), u32>, // city numbers (from, to) -> distance
}

impl Graph<'_> {
    fn minmax_distance(&self) -> (u32, u32) {
        // input (8 citites, 40k paths) is small enough to just brute force
        let mut best = (u32::MAX, u32::MIN);
        for path in (0..self.names.len()).permutations(self.names.len()) {
            let d: u32 = path.windows(2).fold(0, |acc, pair| {
                acc + *self.distance.get(&(pair[0], pair[1])).unwrap()
            });
            if d < best.0 {
                best.0 = d;
            }
            if d > best.1 {
                best.1 = d;
            }
        }
        best
    }
}

impl fmt::Display for Graph<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.names.len() {
            for j in (i + 1)..self.names.len() {
                writeln!(
                    f,
                    "{} to {} = {}",
                    self.names[i],
                    self.names[j],
                    self.distance.get(&(i, j)).unwrap(),
                )?;
            }
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Graph {
    let mut names: Vec<&str> = vec![];
    let mut distance = HashMap::new();

    for line in input.lines() {
        let words: Vec<_> = line.split(' ').collect();
        let city1 = if let Some(pos) = names.iter().position(|name| name == &words[0]) {
            pos
        } else {
            names.push(words[0]);
            names.len() - 1
        };
        let city2 = if let Some(pos) = names.iter().position(|name| name == &words[2]) {
            pos
        } else {
            names.push(words[2]);
            names.len() - 1
        };
        let d = words[4].parse().unwrap();
        distance.insert((city1, city2), d);
        distance.insert((city2, city1), d);
    }

    Graph { names, distance }
}

#[test]
fn test() {
    let test_input = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
";
    let graph = parse_input(test_input);
    assert_eq!((605, 982), graph.minmax_distance());
}
