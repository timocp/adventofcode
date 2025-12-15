use std::collections::HashSet;

pub struct Input {
    replacements: Vec<Replacement>,
    molecule: String,
}

pub fn part1(input: &Input) -> u32 {
    let Input {
        replacements,
        molecule,
    } = input;
    let mut molecules = HashSet::new();
    for replacement in replacements {
        for (pos, _) in molecule.match_indices(&replacement.from) {
            let new_molecule = format!(
                "{}{}{}",
                &molecule[..pos],
                replacement.to,
                &molecule[(pos + replacement.from.len())..]
            );
            molecules.insert(new_molecule);
        }
    }
    molecules.len() as u32
}

pub fn part2(input: &Input) -> u32 {
    let Input {
        replacements,
        molecule,
    } = input;
    let mut answer = None;
    part2_dfs(replacements, molecule, 0, &mut answer);
    answer.unwrap()
}

// this happens to finds my correct answer very quickly but doesn't guarantee the correct answer
// for all inputs because it is not exhaustive.
// TODO: Need something cleverer.
fn part2_dfs(
    replacements: &Vec<Replacement>,
    molecule: &str,
    depth: u32,
    answer: &mut Option<u32>,
) {
    if molecule == "e" {
        *answer = Some(depth);
    } else {
        for replacement in replacements {
            for (pos, _) in molecule.match_indices(&replacement.to) {
                if answer.is_none() {
                    let new_molecule = format!(
                        "{}{}{}",
                        &molecule[..pos],
                        replacement.from,
                        &molecule[(pos + replacement.to.len())..]
                    );
                    part2_dfs(replacements, &new_molecule, depth + 1, answer);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let mut replacements = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        replacements.push(Replacement {
            from: parts.next().unwrap().to_string(),
            to: parts.next().unwrap().to_string(),
        });
    }
    let molecule = lines.next().unwrap().to_string();
    Input {
        replacements,
        molecule,
    }
}

#[test]
fn test_parse_input() {
    let test_input = "H => HO\nH => OH\nO => HH\n\nHOH\n";
    let Input {
        replacements,
        molecule,
    } = parse_input(test_input);
    assert_eq!(replacements.len(), 3);
    assert_eq!(molecule, "HOH");
    assert_eq!(replacements[0].from, "H");
    assert_eq!(replacements[0].to, "HO");
    assert_eq!(replacements[1].from, "H");
    assert_eq!(replacements[1].to, "OH");
    assert_eq!(replacements[2].from, "O");
    assert_eq!(replacements[2].to, "HH");
}

#[test]
fn test_part1() {
    let test_input = "H => HO\nH => OH\nO => HH\n\nHOH\n";
    let input = parse_input(test_input);
    assert_eq!(4, part1(&input));

    let test_input = "H => HO\nH => OH\nO => HH\n\nHOHOHO\n";
    let input = parse_input(test_input);
    assert_eq!(7, part1(&input));
}

#[test]
fn test_part2() {
    let test_input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH\n";
    let input = parse_input(test_input);
    assert_eq!(3, part2(&input));

    let test_input = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO\n";
    let input = parse_input(test_input);
    assert_eq!(6, part2(&input));
}
