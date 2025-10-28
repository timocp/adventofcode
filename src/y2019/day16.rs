pub struct Solver {
    input_signal: Vec<u8>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input_signal: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        repeated_phase(&self.input_signal).to_string()
    }

    fn part2(&self) -> String {
        real_message(&self.input_signal).to_string()
    }
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim_end().chars().map(|c| c as u8 - 48).collect()
}

fn repeated_phase(input: &[u8]) -> u32 {
    let mut buf1: Vec<u8> = input.to_vec();
    let mut buf2: Vec<u8> = input.to_vec();
    for _ in 0..50 {
        phase(&buf1, &mut buf2);
        phase(&buf2, &mut buf1);
    }
    number_from_slice(&buf1[0..8])
}

#[allow(clippy::needless_range_loop)]
fn phase(input: &[u8], output: &mut [u8]) {
    for i in 0..input.len() {
        let mut new_digit = 0i32;

        // positives multiplier groups start at i
        // there are i+1 of them in each group
        // then skip (i+1)*3 elements to find the next group
        let mut index = i;
        while index < input.len() {
            for _ in 0..(i + 1) {
                if index < input.len() {
                    new_digit += input[index] as i32;
                    index += 1;
                }
            }
            index += (i + 1) * 3;
        }

        // negative multiplier groups start at ((i+1)*3)-1
        // there are i+1 of them in each group
        // then skip (i+1)*3 elements to find the next group
        let mut index = (i + 1) * 3 - 1;
        while index < input.len() {
            for _ in 0..(i + 1) {
                if index < input.len() {
                    new_digit -= input[index] as i32;
                    index += 1;
                }
            }
            index += (i + 1) * 3;
        }
        output[i] = (new_digit % 10).unsigned_abs() as u8;
    }
}

// apply 100 phases on the large ("real") signal
// signal is modified in-place during processing
// returns the 8-digit message found at the offset
fn real_message(input: &[u8]) -> u32 {
    // generate the "real" signal
    let mut signal = input.repeat(10000);

    let offset = number_from_slice(&signal[0..7]) as usize;

    // if offset > half the input length, all multiplies are 1
    // we don't need to calculate any digits before the offset
    for _ in 0..100 {
        let mut acc = 0i32;
        for i in (offset..(signal.len() - 1)).rev() {
            acc += signal[i] as i32;
            signal[i] = (acc.abs() % 10) as u8;
        }
    }

    number_from_slice(&signal[offset..(offset + 8)])
}

fn number_from_slice(data: &[u8]) -> u32 {
    data.iter().fold(0, |acc, digit| acc * 10 + *digit as u32)
}

#[test]
fn test_phase() {
    let mut buf1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut buf2 = vec![0, 0, 0, 0, 0, 0, 0, 0];
    phase(&buf1, &mut buf2);
    assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8], buf2);
    phase(&buf2, &mut buf1);
    assert_eq!(vec![3, 4, 0, 4, 0, 4, 3, 8], buf1);
    phase(&buf1, &mut buf2);
    assert_eq!(vec![0, 3, 4, 1, 5, 5, 1, 8], buf2);
    phase(&buf2, &mut buf1);
    assert_eq!(vec![0, 1, 0, 2, 9, 4, 9, 8], buf1);
}

#[test]
fn test_repeated_phase() {
    assert_eq!(
        24176176,
        repeated_phase(&parse_input("80871224585914546619083218645595"))
    );
    assert_eq!(
        73745418,
        repeated_phase(&parse_input("19617804207202209144916044189917"))
    );
    assert_eq!(
        52432133,
        repeated_phase(&parse_input("69317163492948606335995924319873"))
    );
}

#[test]
fn test_real_message() {
    let example1 = "03036732577212944063491565474664";
    assert_eq!(84462026, real_message(&parse_input(example1)));

    let example2 = "02935109699940807407585447034323";
    assert_eq!(78725270, real_message(&parse_input(example2)));

    let example3 = "03081770884921959731165446850517";
    assert_eq!(53553731, real_message(&parse_input(example3)));
}
