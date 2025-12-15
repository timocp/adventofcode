use md5::{Digest, Md5};

pub fn parse_input(input: &str) -> String {
    input.trim().to_string()
}

pub fn part1(door_id: &str) -> String {
    bf_password1(door_id)
}

pub fn part2(door_id: &str) -> String {
    bf_password2(door_id)
}

fn bf_password1(door_id: &str) -> String {
    let mut password = String::with_capacity(8);
    let mut hasher = Md5::new();

    // store the number as ascii digits, so that it can be passed directly to the hasher.  this is
    // about 20% faster than storing it as an integer and converting it to a string each time we
    // need to hash it.
    let mut i: Vec<u8> = vec![b'0'];

    for _ in 0..8 {
        loop {
            hasher.update(door_id);
            hasher.update(&i);
            let hash = hasher.finalize_reset();
            increment(&mut i);

            if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
                password.push(digit_to_ascii(hash[2]) as char);
                break;
            }
        }
    }

    password
}

fn bf_password2(door_id: &str) -> String {
    let mut password = [0u8; 8];
    let mut hasher = Md5::new();

    let mut i: Vec<u8> = vec![b'0'];

    for _ in 0..8 {
        loop {
            hasher.update(door_id);
            hasher.update(&i);
            let hash = hasher.finalize_reset();
            increment(&mut i);

            if hash[0] == 0 && hash[1] == 0 && hash[2] < 8 && password[hash[2] as usize] == 0 {
                password[hash[2] as usize] = digit_to_ascii(hash[3] >> 4);
                break;
            }
        }
    }

    String::from_utf8_lossy(&password).to_string()
}

// increment a number stored as a Vec of ascii digits
fn increment(i: &mut Vec<u8>) {
    for digit in i.iter_mut().rev() {
        if *digit < b'9' {
            *digit += 1;
            return;
        } else {
            *digit = b'0';
        }
    }
    i.insert(0, b'1');
}

fn digit_to_ascii(digit: u8) -> u8 {
    digit + if digit < 10 { b'0' } else { b'a' - 10 }
}

#[test]
#[ignore] // slow.  run with: cargo test --release -- --ignored
fn test() {
    assert_eq!("18f47a30", bf_password1("abc"));
    assert_eq!("05ace8e3", bf_password2("abc"));
}
