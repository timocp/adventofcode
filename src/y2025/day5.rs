#[derive(Debug)]
pub struct Input {
    fresh_ranges: Vec<(u64, u64)>,
    ingredients: Vec<u64>,
}

pub fn parse_input(input: &str) -> Input {
    let mut fresh_ranges = vec![];
    let mut ingredients = vec![];

    for line in input.lines().filter(|&line| !line.is_empty()) {
        if let Some(hyphen) = line.find('-') {
            let range = line.split_at(hyphen);
            fresh_ranges.push((range.0.parse().unwrap(), (range.1[1..]).parse().unwrap()));
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    Input {
        fresh_ranges,
        ingredients,
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|&&ingredient| {
            input
                .fresh_ranges
                .iter()
                .any(|range| ingredient >= range.0 && ingredient <= range.1)
        })
        .count()
}

pub fn part2(input: &Input) -> u64 {
    let ranges = &input.fresh_ranges;
    // track an array of unfresh regions an ranges starting with all integers and split as fresh ranges known
    // i don't know why but i thought this would be easier than tracking fresh explicitly?
    let mut unfresh: Vec<(u64, u64)> = vec![(0, u64::MAX)];
    for &(low, high) in ranges {
        unfresh = unfresh
            .into_iter()
            .flat_map(|(start, end)| {
                if low < start {
                    if high > end {
                        vec![]
                    } else if high >= start {
                        vec![(high + 1, end)]
                    } else {
                        vec![(start, end)]
                    }
                } else if low == start {
                    if high < end {
                        vec![(high + 1, end)]
                    } else {
                        vec![]
                    }
                } else if low < end {
                    if high < end {
                        vec![(start, low - 1), (high + 1, end)]
                    } else {
                        vec![(start, low - 1)]
                    }
                } else if low == end {
                    vec![(start, low - 1)]
                } else {
                    vec![(start, end)]
                }
            })
            .collect();
    }

    u64::MAX - unfresh.iter().map(|r| r.1 - r.0 + 1).sum::<u64>() + 1
}

#[test]
fn test() {
    let test_input = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    let input = parse_input(test_input);
    assert_eq!(3, part1(&input));
    assert_eq!(14, part2(&input));
}
