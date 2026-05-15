use crate::pos3d::Pos3d;

pub fn parse_input(input: &str) -> Vec<Pos3d> {
    input.lines().map(Pos3d::from).collect()
}

pub fn part1(input: &[Pos3d]) -> u32 {
    solve_part1(input, 1000)
}

fn solve_part1(input: &[Pos3d], connections: usize) -> u32 {
    // Vec storing which circuit each junction box is in
    // Each junction box starts in a circuit containing itself
    let mut circuit: Vec<usize> = (0..(input.len())).collect();

    // Prepare a list of pairs of junction boxes sorted by distance
    let mut pairs: Vec<(usize, usize, i64)> = vec![];
    for (i, a) in input.iter().enumerate() {
        for (j, b) in input.iter().enumerate().skip(i + 1) {
            pairs.push((i, j, a.distance_cmp(b)));
        }
    }
    pairs.sort_by_key(|a| a.2);

    for (i, j, _) in pairs.iter().take(connections) {
        let from = circuit[*i];
        let to = circuit[*j];
        if from != to {
            // println!("linking {} (circuit {}) to {} (circuit {})", i, from, j, to);
            for c in circuit.iter_mut() {
                if *c == from {
                    *c = to;
                }
            }
        } else {
            // println!("{} and {} are already in the same circuit ({})", i, j, from);
        }
    }

    // find the 3 biggest circuits
    let mut sizes: Vec<_> = circuit.iter().map(|_| 0).collect();
    for i in 0..circuit.len() {
        sizes[circuit[i]] += 1;
    }
    sizes.sort_by(|a, b| b.cmp(a));

    // multiply sizes of the biggest 3
    sizes.into_iter().take(3).product()
}

pub fn part2(_input: &[Pos3d]) -> &str {
    "unimplemented"
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
    assert_eq!(40, solve_part1(&parse_input(test_input), 10));
}
