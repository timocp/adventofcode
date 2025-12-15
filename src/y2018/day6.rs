use crate::grid::Pos;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn part1(points: &[Pos]) -> usize {
    largest_finite_area(points)
}

pub fn part2(points: &[Pos]) -> usize {
    safe_area(points, 10000)
}

#[derive(Debug)]
struct Closest {
    distance: usize,
    coord: Option<usize>,
}

fn largest_finite_area(coords: &[Pos]) -> usize {
    let (min, max) = box_size(coords);

    let mut closest: HashMap<Pos, Closest> = HashMap::new();
    for (c, coord) in coords.iter().enumerate() {
        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let distance = ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                match closest.entry(Pos { x, y }) {
                    Entry::Occupied(ent) => {
                        let ent = ent.into_mut();
                        if distance < ent.distance {
                            ent.distance = distance;
                            ent.coord = Some(c);
                        } else if distance == ent.distance {
                            ent.coord = None;
                        }
                    }
                    Entry::Vacant(ent) => {
                        ent.insert(Closest {
                            distance,
                            coord: Some(c),
                        });
                    }
                }
            }
        }
    }

    // areas, Some(x) means finite area x, None means infinite
    let mut areas: Vec<Option<usize>> = vec![Some(0); coords.len()];
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let closest = closest.get(&Pos { x, y }).unwrap();
            if let Some(c) = closest.coord {
                if y == min.y || y == max.y || x == min.x || x == max.x {
                    areas[c] = None;
                } else if let Some(v) = areas[c] {
                    areas[c] = Some(v + 1);
                }
            }
        }
    }
    areas.into_iter().flatten().max().unwrap()
}

fn safe_area(coords: &[Pos], limit: usize) -> usize {
    let (min, max) = box_size(coords);
    let mut area = 0;
    for x in min.x..=max.x {
        'cell: for y in min.y..=max.y {
            let mut distance = 0;
            for coord in coords.iter() {
                distance += ((coord.x - x).abs() + (coord.y - y).abs()) as usize;
                if distance >= limit {
                    continue 'cell;
                }
            }
            area += 1;
        }
    }
    area
}

fn box_size(coords: &[Pos]) -> (Pos, Pos) {
    let mut min = *coords.first().unwrap();
    let mut max = *coords.first().unwrap();
    for c in coords.iter().skip(1) {
        if c.x < min.x {
            min.x = c.x;
        } else if c.x > max.x {
            max.x = c.x;
        }
        if c.y < min.y {
            min.y = c.y;
        } else if c.y > max.y {
            max.y = c.y;
        }
    }
    (min, max)
}

pub fn parse_input(input: &str) -> Vec<Pos> {
    let mut list = vec![];
    for line in input.lines() {
        let coords: Vec<_> = line.split(", ").filter_map(|s| s.parse().ok()).collect();
        list.push(Pos {
            x: coords[0],
            y: coords[1],
        });
    }
    list
}

#[test]
fn test_run() {
    let test_input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(17, largest_finite_area(&parse_input(test_input)));
    assert_eq!(16, safe_area(&parse_input(test_input), 32));
}
