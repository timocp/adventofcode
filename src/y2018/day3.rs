use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Solver {
    claims: Vec<Claim>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            claims: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        let mut fabric = HashMap::new();
        process(&mut fabric, &self.claims).to_string()
    }

    fn part2(&self) -> String {
        let mut fabric = HashMap::new();
        let _ = process(&mut fabric, &self.claims);
        intact_claim(&fabric, &self.claims).to_string()
    }
}

fn process(fabric: &mut HashMap<(usize, usize), Square>, claims: &Vec<Claim>) -> i32 {
    let mut overlap_count = 0;
    for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                match fabric.entry((x, y)) {
                    Entry::Occupied(ent) => {
                        let sq = ent.into_mut();
                        if !sq.overlaps {
                            overlap_count += 1;
                            sq.overlaps = true;
                        }
                        sq.content = claim.id;
                    }
                    Entry::Vacant(ent) => {
                        ent.insert(Square {
                            content: claim.id,
                            overlaps: false,
                        });
                    }
                }
            }
        }
    }
    overlap_count
}

fn intact_claim(fabric: &HashMap<(usize, usize), Square>, claims: &Vec<Claim>) -> usize {
    'claim: for claim in claims {
        for x in claim.left..(claim.left + claim.width) {
            for y in claim.top..(claim.top + claim.height) {
                if let Some(sq) = fabric.get(&(x, y))
                    && sq.overlaps
                {
                    continue 'claim;
                }
            }
        }
        return claim.id;
    }
    0
}

fn parse_input(input: &str) -> Vec<Claim> {
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims = vec![];
    for line in input.lines() {
        match re.captures(line) {
            Some(cap) => claims.push(Claim {
                id: cap[1].parse().unwrap(),
                left: cap[2].parse().unwrap(),
                top: cap[3].parse().unwrap(),
                width: cap[4].parse().unwrap(),
                height: cap[5].parse().unwrap(),
            }),
            None => eprintln!("parse error: {}", line),
        }
    }
    claims
}

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
struct Square {
    content: usize,
    overlaps: bool,
}

#[test]
fn test_run() {}
