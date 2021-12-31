use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let pw = parse_input(input);
    match part {
        Part::One => password_to_s(&next_password(&pw)),
        Part::Two => "?".to_string(),
    }
}

type Password = [u8; 8];

fn valid_password(pw: &Password) -> bool {
    // at least one increasing straight of at least 3 letters
    if !pw.windows(3).any(|b| b[0] == b[1] - 1 && b[1] == b[2] - 1) {
        return false;
    }

    // may not contain i, o or l
    if pw.iter().any(|&b| b == b'i' || b == b'o' || b == b'l') {
        return false;
    }

    // must contain at least 2 different non-overlapping pairs
    for i in 0..6 {
        for j in (i + 2)..7 {
            if pw[i] == pw[i + 1] && pw[j] == pw[j + 1] {
                return true;
            }
        }
    }
    false
}

fn increment_password(pw: &mut Password) {
    for i in (0..8).rev() {
        if pw[i] == b'z' {
            pw[i] = b'a';
        } else {
            pw[i] += 1;
            return;
        }
    }
    panic!("tried to increment past end of range");
}

fn next_password(pw: &Password) -> Password {
    let mut new = *pw;
    loop {
        increment_password(&mut new);
        if valid_password(&new) {
            return new;
        }
    }
}

fn password_to_s(pw: &Password) -> String {
    String::from_utf8(pw.iter().copied().collect::<Vec<_>>()).unwrap()
}

fn parse_input(input: &str) -> Password {
    let mut p = [0; 8];
    for (i, &b) in input.lines().next().unwrap().as_bytes().iter().enumerate() {
        p[i] = b;
    }
    p
}

#[test]
fn test() {
    assert!(!valid_password(&parse_input("hijklmmn")));
    assert!(!valid_password(&parse_input("abbceffg")));
    assert!(!valid_password(&parse_input("abbcegjk")));

    assert_eq!(
        parse_input("abcdffaa"),
        next_password(&parse_input("abcdefgh"))
    );
    assert_eq!(
        parse_input("ghjaabcc"),
        next_password(&parse_input("ghijklmn"))
    );
}
