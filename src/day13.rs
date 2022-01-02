use crate::Part;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let diners = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => diners.happiest(),
            Part::Two => 0,
        }
    )
}

struct Diners {
    names: Vec<String>,
    happiness: HashMap<(usize, usize), i32>,
}

impl Diners {
    fn happy_pair(&self, p1: usize, p2: usize) -> i32 {
        *self.happiness.get(&(p1, p2)).unwrap() + *self.happiness.get(&(p2, p1)).unwrap()
    }

    fn happiest(&self) -> i32 {
        let mut best = i32::MIN;

        for seating in (0..self.names.len())
            .permutations(self.names.len())
            .filter(|seating| seating[0] == 0)
        {
            let d = seating
                .windows(2)
                .fold(0, |acc, pair| acc + self.happy_pair(pair[0], pair[1]))
                + self.happy_pair(seating[0], seating[seating.len() - 1]);
            if d > best {
                best = d;
            }
        }

        best
    }
}

impl fmt::Debug for Diners {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        dbg!(&self.names);
        for i in 0..self.names.len() {
            for j in 0..self.names.len() {
                if i == j {
                    continue;
                }
                let h = *self.happiness.get(&(i, j)).unwrap();
                if h >= 0 {
                    writeln!(
                        f,
                        "{} would gain {} happiness units by sitting next to {}",
                        self.names[i], h, self.names[j]
                    )?;
                } else {
                    writeln!(
                        f,
                        "{} would lose {} happiness units by sitting next to {}",
                        self.names[i], -h, self.names[j]
                    )?;
                }
            }
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Diners {
    let mut names: Vec<String> = vec![];
    let mut happiness = HashMap::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$"
        )
        .unwrap();
    }

    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let p1 = if let Some(pos) = names.iter().position(|name| name == &caps[1]) {
            pos
        } else {
            names.push(caps[1].to_string());
            names.len() - 1
        };
        let p2 = if let Some(pos) = names.iter().position(|name| name == &caps[4]) {
            pos
        } else {
            names.push(caps[4].to_string());
            names.len() - 1
        };
        let h = caps[3].parse::<i32>().unwrap();
        happiness.insert(
            (p1, p2),
            match &caps[2] {
                "gain" => h,
                "lose" => -h,
                _ => panic!(),
            },
        );
    }

    Diners { names, happiness }
}

#[test]
fn test() {
    let test_input = "\
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
    let diners = parse_input(test_input);
    assert_eq!(330, diners.happiest());
}
