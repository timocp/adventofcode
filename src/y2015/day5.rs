use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let words = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&words),
            Part::Two => part2(&words),
        }
    )
}

fn count_vowels(chars: &[char]) -> usize {
    chars
        .iter()
        .filter(|&&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
}

fn contains_double(chars: &[char]) -> bool {
    chars.windows(2).any(|pair| pair[0] == pair[1])
}

fn contains_bad(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn is_nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    count_vowels(&chars) >= 3 && contains_double(&chars) && !contains_bad(s)
}

fn contains_non_overlapping_pair(chars: &[char]) -> bool {
    for i in 0..(chars.len() - 2) {
        let pair = (chars[i], chars[i + 1]);
        for j in (i + 2)..(chars.len() - 1) {
            if pair.0 == chars[j] && pair.1 == chars[j + 1] {
                return true;
            }
        }
    }
    false
}

fn contains_sandwiched_letter(chars: &[char]) -> bool {
    chars.windows(3).any(|slice| slice[0] == slice[2])
}

fn is_nice2(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    contains_non_overlapping_pair(&chars) && contains_sandwiched_letter(&chars)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(words: &[&str]) -> usize {
    words.iter().filter(|word| is_nice(word)).count()
}

fn part2(words: &[&str]) -> usize {
    words.iter().filter(|word| is_nice2(word)).count()
}

#[test]
fn test() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));

    assert!(is_nice2("qjhvhtzxzqqjkmpb"));
    assert!(is_nice2("xxyxx"));
    assert!(!is_nice2("uurcxstgmygtbstg"));
    assert!(!is_nice2("ieodomkazucvgmuy"));
}
