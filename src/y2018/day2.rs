use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

pub fn part1(input: &[String]) -> i32 {
    checksum(input)
}

pub fn part2(input: &[String]) -> String {
    common_letters(input)
}

fn checksum(input: &[String]) -> i32 {
    let mut t2 = 0;
    let mut t3 = 0;

    let mut map = HashMap::new();
    for id in input {
        map.clear();
        for c in id.chars() {
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }
        if map.values().any(|v| *v == 2) {
            t2 += 1;
        }
        if map.values().any(|v| *v == 3) {
            t3 += 1;
        }
    }
    t2 * t3
}

fn common_letters(input: &[String]) -> String {
    for (i, id1) in input.iter().enumerate() {
        for id2 in input.iter().skip(i + 1) {
            if let Some(s) = compare(id1, id2) {
                return s;
            }
        }
    }
    String::new()
}

// Return common characters for 2 strings, as long as there is exactly 1 difference
fn compare(a: &str, b: &str) -> Option<String> {
    let mut result = String::new();
    let mut mismatches = false;
    for (c1, c2) in a.chars().zip(b.chars()) {
        if c1 == c2 {
            result.push(c1);
        } else if mismatches {
            return None;
        } else {
            mismatches = true;
        }
    }
    if mismatches { Some(result) } else { None }
}

#[test]
fn test_checksum() {
    assert_eq!(
        12,
        checksum(&vec![
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned()
        ])
    );
}

#[test]
fn test_common_letters() {
    assert_eq!(
        "fgij",
        common_letters(&vec![
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned()
        ])
    );
}
