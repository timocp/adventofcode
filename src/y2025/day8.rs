use crate::pos3d::Pos3d;
use crate::union_find::UnionFind;

pub fn part1(input: &Input) -> usize {
    solve_part1(input, 1000)
}

pub struct Input {
    boxes: Vec<Pos3d>,
    connections: Vec<Connection>,
}

#[derive(Debug)]
struct Connection {
    from: usize,
    to: usize,
    distance: i64,
}

fn solve_part1(input: &Input, count: usize) -> usize {
    let mut circuits = UnionFind::new(input.boxes.len());

    for conn in input.connections.iter().take(count) {
        circuits.union(conn.from, conn.to);
    }

    // find the 3 biggest circuits
    let mut sizes: Vec<_> = (0..input.boxes.len())
        .filter_map(|n| circuits.size_of(n))
        .collect();
    sizes.sort_by(|a, b| b.cmp(a));

    // multiply sizes of the biggest 3
    sizes.into_iter().take(3).product()
}

pub fn part2(input: &Input) -> i64 {
    // for part 2, if there are 1000 circuits to start with, we need to
    // perform a link 999 times
    let mut circuits = UnionFind::new(input.boxes.len());
    let mut links = 0;

    for conn in input.connections.iter() {
        if circuits.union(conn.from, conn.to) {
            links += 1;
            if links == input.boxes.len() - 1 {
                return input.boxes[conn.from].x as i64 * input.boxes[conn.to].x as i64;
            }
        }
    }

    unreachable!()
}

pub fn parse_input(input: &str) -> Input {
    let boxes: Vec<Pos3d> = input.lines().map(Pos3d::from).collect();

    let mut connections: Vec<Connection> = vec![];

    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().enumerate().skip(i + 1) {
            connections.push(Connection {
                from: i,
                to: j,
                distance: a.distance_cmp(b),
            })
        }
    }
    connections.sort_by_key(|a| a.distance);

    Input { boxes, connections }
}

#[test]
fn test() {
    let test_input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    let input = parse_input(test_input);
    assert_eq!(40, solve_part1(&input, 10));
    assert_eq!(25272, part2(&input));
}
