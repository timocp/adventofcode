use crate::grid::parse_each_char;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Input {
    // index of beam start
    start: usize,
    // array of indexes of splitter for each line
    splitters: Vec<HashSet<usize>>,
}

pub fn parse_input(input: &str) -> Input {
    let mut start = 0;
    let mut splitters: Vec<HashSet<usize>> = vec![];
    for (p, c) in parse_each_char(input) {
        if p.x == 0 {
            splitters.push(HashSet::new());
        }
        if c == 'S' {
            start = p.x as usize;
        } else if c == '^' {
            splitters[p.y as usize].insert(p.x as usize);
        }
    }
    Input { start, splitters }
}

pub fn part1(input: &Input) -> usize {
    let mut beams: HashSet<usize> = HashSet::from([input.start]);
    let mut count = 0;
    for splits in input.splitters.iter() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for beam in beams.into_iter() {
            if splits.contains(&beam) {
                count += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(beam);
            }
        }
        beams = new_beams;
    }
    count
}

pub fn part2(input: &Input) -> &str {
    "unimplemented"
}

#[test]
fn test() {
    let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    let input = parse_input(test_input);
    assert_eq!(21, part1(&input));
}
