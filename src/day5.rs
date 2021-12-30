use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let words = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&words),
            Part::Two => 0,
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
    for i in 0..(chars.len() - 1) {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn contains_bad(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn is_nice(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    count_vowels(&chars) >= 3 && contains_double(&chars) && !contains_bad(s)
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(words: &[&str]) -> usize {
    words.iter().filter(|word| is_nice(word)).count()
}

#[test]
fn test() {
    assert!(is_nice("ugknbfddgicrmopn"));
    assert!(is_nice("aaa"));
    assert!(!is_nice("jchzalrnumimnmhp"));
    assert!(!is_nice("haegwjzuvuyypxyu"));
    assert!(!is_nice("dvszwmarrgswjxmb"));
}
