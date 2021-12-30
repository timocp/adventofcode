use crate::Part;
use std::sync::mpsc;
use std::thread;

pub fn run(input: &str, part: Part) -> String {
    let key = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&key),
            Part::Two => part2(&key),
        }
    )
}

fn parse_input(input: &str) -> String {
    input.lines().next().unwrap().to_string()
}

fn search_hash(key: &str, zeros: u32) -> u32 {
    let mut handles = vec![];
    // channel used for threads to tell main when they have a result
    let (tx, rx) = mpsc::channel();
    let threads = num_cpus::get() as u32;
    for i in 0..threads {
        let tx = tx.clone();
        let key = key.to_owned();
        // channel used by main to tell threads a result has been found
        let (tstop, rstop) = mpsc::channel();
        handles.push((
            thread::spawn(move || {
                let mut best: u32 = u32::MAX;
                let mut num = i;
                while num < best {
                    let digest = md5::compute(format!("{}{}", key, num).as_bytes());
                    if match zeros {
                        5 => digest[0] == 0 && digest[1] == 0 && digest[2] < 16,
                        6 => digest[0] == 0 && digest[1] == 0 && digest[2] == 0,
                        _ => unreachable!(),
                    } {
                        let _ = tx.send(num); // don't care if error
                        return Some(num);
                    }
                    num += threads;
                    if let Ok(n) = rstop.try_recv() {
                        best = n;
                    }
                }
                None
            }),
            tstop,
        ));
    }

    // wait until first result has come in
    let candidate = rx.recv().unwrap();

    // send this found number to all threads; they will stop as soon as their search couldn't
    // possibly find anything better.
    for h in &handles {
        let _ = h.1.send(candidate); // don't care if error because thread has exited
    }

    // wait for all threads to finish what they're doing then return the lowest number found.
    // there could be multiple matches if there are results nearby and the wrong number was found
    // faster than the best one.
    handles
        .into_iter()
        .filter_map(|jh| jh.0.join().unwrap())
        .min()
        .unwrap()
}

fn part1(key: &str) -> u32 {
    search_hash(key, 5)
}

fn part2(key: &str) -> u32 {
    search_hash(key, 6)
}

#[test]
fn test() {
    assert_eq!(609043, part1(&parse_input("abcdef\n")));
    assert_eq!(1048970, part1(&parse_input("pqrstuv\n")));
}
