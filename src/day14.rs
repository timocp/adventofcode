use crate::Part;
use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: &str, part: Part) -> String {
    let reindeer = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&reindeer, 2503),
            Part::Two => part2(&reindeer, 2503),
        }
    )
}

fn part1(reindeer: &[Reindeer], seconds: u32) -> u32 {
    simulate(reindeer, seconds)
        .iter()
        .map(|state| state.distance)
        .max()
        .unwrap()
}

fn part2(reindeer: &[Reindeer], seconds: u32) -> u32 {
    simulate(reindeer, seconds)
        .iter()
        .map(|state| state.score)
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,         // km/s
    flight_period: u32, // seconds
    rest_period: u32,   // seconds
}

#[derive(Debug)]
enum Movement {
    Flying(u32),
    Resting(u32),
}

struct State {
    movement: Movement,
    distance: u32, // km
    score: u32,
}

fn simulate(reindeer: &[Reindeer], seconds: u32) -> Vec<State> {
    let mut state: Vec<State> = reindeer
        .iter()
        .map(|r| State {
            movement: Movement::Flying(r.flight_period),
            distance: 0,
            score: 0,
        })
        .collect();
    let mut time = 0;

    while time < seconds {
        let mut leading_km = 0;
        time += 1;
        for (i, r) in reindeer.iter().enumerate() {
            state[i].movement = match state[i].movement {
                Movement::Flying(t) => {
                    state[i].distance += r.speed;
                    if t > 1 {
                        Movement::Flying(t - 1)
                    } else {
                        Movement::Resting(r.rest_period)
                    }
                }
                Movement::Resting(t) => {
                    if t > 1 {
                        Movement::Resting(t - 1)
                    } else {
                        Movement::Flying(r.flight_period)
                    }
                }
            };
            if state[i].distance > leading_km {
                leading_km = state[i].distance;
            }
        }
        for s in state.iter_mut().filter(|s| s.distance == leading_km) {
            s.score += 1;
        }
        // for (i, r) in reindeer.iter().enumerate() {
        //     println!(
        //         "After {} seconds, {} had gone {} km and is {:?} with score {}",
        //         time, r.name, state[i].distance, state[i].movement, state[i].score
        //     );
        // }
    }

    state
}

impl From<&str> for Reindeer {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.$").unwrap();
        }
        let m = RE.captures(s).unwrap();
        Self {
            name: m[1].to_string(),
            speed: m[2].parse().unwrap(),
            flight_period: m[3].parse().unwrap(),
            rest_period: m[4].parse().unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input.lines().map(|line| Reindeer::from(line)).collect()
}

#[test]
fn test() {
    let test_input = "\
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
";
    let reindeer = parse_input(test_input);
    assert_eq!(1120, part1(&reindeer, 1000));
    assert_eq!(689, part2(&reindeer, 1000));
}
