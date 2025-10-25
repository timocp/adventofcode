pub struct Solver {
    input_signal: Vec<i8>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input_signal: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        first_digits(&repeated_phase(&self.input_signal))
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn parse_input(input: &str) -> Vec<i8> {
    input.trim_end().chars().map(|c| c as i8 - 48).collect()
}

fn first_digits(signal: &[i8]) -> String {
    format!(
        "{}{}{}{}{}{}{}{}",
        signal[0], signal[1], signal[2], signal[3], signal[4], signal[5], signal[6], signal[7]
    )
}

fn repeated_phase(input: &[i8]) -> Vec<i8> {
    let mut buf1: Vec<i8> = input.to_vec();
    let mut buf2: Vec<i8> = input.to_vec();
    for _ in 0..50 {
        phase(&buf1, &mut buf2);
        phase(&buf2, &mut buf1);
    }
    buf1
}

fn phase(input: &[i8], output: &mut [i8]) {
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
        output[i] = (new_digit % 10).abs() as i8;
    }
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
        "24176176",
        first_digits(&repeated_phase(&parse_input(
            "80871224585914546619083218645595"
        )))
    );
    assert_eq!(
        "73745418",
        first_digits(&repeated_phase(&parse_input(
            "19617804207202209144916044189917"
        )))
    );
    assert_eq!(
        "52432133",
        first_digits(&repeated_phase(&parse_input(
            "69317163492948606335995924319873"
        )))
    );
}
