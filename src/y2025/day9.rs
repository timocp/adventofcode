use crate::image::{Colour, Image};
use crate::pos::Pos;
use std::cmp::Reverse;
use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<Pos> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: &[Pos]) -> u64 {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, a)| input.iter().skip(i + 1).map(|b| area(a, b)))
        .max()
        .unwrap()
}

fn area(a: &Pos, b: &Pos) -> u64 {
    ((a.x - b.x).unsigned_abs() as u64 + 1) * ((a.y - b.y).unsigned_abs() as u64 + 1)
}

pub fn part2(input: &[Pos]) -> u64 {
    let coords = compact_axes(&input);

    let boundary = Boundary::new(&coords.iter().map(|c| c.compacted).collect::<Vec<_>>());

    // 9 mins to get:
    // checking 213080 points in rect 5424,67429..94862,50327 cache.size=16437556
    // after compacting indices:
    // 0.3s to get:
    // checking 438 points in rect 29,153..218,123 cache.size=35376
    let mut cache: HashMap<Pos, bool> = HashMap::new();

    // (a, b, area):  compacted (a,b) but original area
    let mut rects = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords
                .iter()
                .skip(i + 1)
                .map(move |b| (&a.compacted, &b.compacted, area(&a.original, &b.original)))
        })
        .collect::<Vec<_>>();

    rects.sort_by_key(|d| Reverse(d.2));

    rects
        .iter()
        .find(|&&(a, b, _)| boundary.contains_rect(a, b, &mut cache))
        .unwrap()
        .2
}

#[derive(Debug)]
struct CompactedPos {
    original: Pos,
    compacted: Pos,
}

fn compact_axes(original: &[Pos]) -> Vec<CompactedPos> {
    let compact_x = compact_indices(&original.iter().map(|p| p.x).collect::<Vec<i32>>());
    let compact_y = compact_indices(&original.iter().map(|p| p.y).collect::<Vec<i32>>());

    let compacted = original
        .iter()
        .map(|p| CompactedPos {
            original: p.clone(),
            compacted: Pos {
                x: *compact_x.get(&p.x).unwrap(),
                y: *compact_y.get(&p.y).unwrap(),
            },
        })
        .collect();

    compacted
}

// returns map of original index -> compacted index
fn compact_indices(indexes: &[i32]) -> HashMap<i32, i32> {
    let mut all: Vec<_> = indexes.iter().collect();
    all.sort();
    all.dedup();
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, &x) in all.iter().enumerate() {
        map.insert(*x, i.try_into().unwrap());
    }

    map
}

struct Boundary {
    lines: Vec<Line>,
}

impl Boundary {
    fn new(points: &[Pos]) -> Self {
        let mut lines: Vec<_> = points
            .windows(2)
            .map(|w| Line {
                from: w[0],
                to: w[1],
            })
            .collect();

        lines.push(Line {
            from: points[points.len() - 1],
            to: points[0],
        });

        // normalise lines to left->right or up->down
        for line in lines.iter_mut() {
            if line.from.x > line.to.x || line.from.y > line.to.y {
                (line.from, line.to) = (line.to, line.from);
            }
        }

        Self { lines }
    }

    // For each point on a candidate rectangle, check if:
    // (a) it is on a boundary line, or
    // (b) raycasts with an odd number of intersections with boundary lines
    fn contains_rect(&self, a: &Pos, b: &Pos, cache: &mut HashMap<Pos, bool>) -> bool {
        let points = rect_points(a, b);
        // println!(
        //     "checking {} points in rect {}..{} cache.size={}",
        //     points.len(),
        //     a,
        //     b,
        //     cache.iter().count()
        // );
        points.iter().all(|p| self.contains_point(p, cache))
    }

    fn contains_point(&self, p: &Pos, cache: &mut HashMap<Pos, bool>) -> bool {
        *cache
            .entry(*p)
            .or_insert_with(|| self.is_on_boundary(p) || self.is_internal(p))
    }

    fn is_on_boundary(&self, p: &Pos) -> bool {
        self.lines.iter().any(|l| {
            if l.horizontal() {
                p.y == l.from.y && p.x >= l.from.x && p.x <= l.to.x
            } else {
                p.x == l.from.x && p.y >= l.from.y && p.y <= l.to.y
            }
        })
    }

    fn is_internal(&self, p: &Pos) -> bool {
        // trace left and count the number of lines we cross
        // if is is odd, we must be inside the polygon
        let count = self
            .lines
            .iter()
            .filter(|&l| {
                l.to.x < p.x
                    && if l.vertical() {
                        p.y >= l.from.y && p.y <= l.to.y
                    } else {
                        p.y == l.from.y
                    }
            })
            .count();

        count % 2 == 1
    }
}

// return all points on a rectable
//  a---d
//  |   |  c is (a.x, b.y)
//  |   |  d is (b.x, a.y)
//  c---b
fn rect_points(a: &Pos, b: &Pos) -> Vec<Pos> {
    // normalise so A is top-left and B is bottom-right
    let (minx, maxx) = minmax(a.x, b.x);
    let (miny, maxy) = minmax(a.y, b.y);
    let a = Pos { x: minx, y: miny };
    let b = Pos { x: maxx, y: maxy };

    if a.y == b.y {
        (minx..=maxx).map(|x| Pos { x, y: a.y }).collect()
    } else if a.x == b.x {
        (miny..=maxy).map(|y| Pos { x: a.x, y }).collect()
    } else {
        let mut points: Vec<Pos> = vec![];
        for x in minx..=maxx {
            points.push(Pos { x, y: miny });
            points.push(Pos { x, y: maxy });
        }
        for y in (miny + 1)..maxy {
            points.push(Pos { x: minx, y });
            points.push(Pos { x: maxx, y });
        }
        points
    }
}

fn minmax(a: i32, b: i32) -> (i32, i32) {
    if a < b { (a, b) } else { (b, a) }
}
pub fn visualise(input: &[Pos]) {
    let minx = input.iter().map(|p| p.x).min().unwrap();
    let maxx = input.iter().map(|p| p.x).max().unwrap();
    let miny = input.iter().map(|p| p.y).min().unwrap();
    let maxy = input.iter().map(|p| p.y).max().unwrap();

    let lines = Boundary::new(input).lines;

    let mut image = Image::new(&(minx, miny).into(), &(maxx, maxy).into());

    for line in lines.iter() {
        if line.vertical() {
            let (min, max) = minmax(line.from.y, line.to.y);
            for y in min..=max {
                image.set(&(line.to.x, y).into(), Colour::Green);
            }
        } else {
            let (min, max) = minmax(line.from.x, line.to.x);
            for x in min..=max {
                image.set(&(x, line.to.y).into(), Colour::Green);
            }
        }
    }
    for corner in input.iter() {
        image.set(corner, Colour::Red);
    }

    image.save("tmp/2025day9.png").unwrap();
}

#[derive(Debug)]
struct Line {
    from: Pos,
    to: Pos,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn vertical(&self) -> bool {
        self.from.x == self.to.x
    }
}

#[test]
fn test() {
    let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    let input = parse_input(test_input);
    assert_eq!(50, part1(&input));
    assert_eq!(24, part2(&input));
}
