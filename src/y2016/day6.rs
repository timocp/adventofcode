pub fn part1(codes: &[String]) -> String {
    most_common_letters(codes)
}

pub fn part2(codes: &[String]) -> String {
    least_common_letters(codes)
}

fn most_common_letters(codes: &[String]) -> String {
    count_letters(codes)
        .iter()
        .map(|count| {
            let mut max = 0;
            let mut max_index = 0;
            for (i, &c) in count.iter().enumerate() {
                if c > max {
                    max = c;
                    max_index = i;
                }
            }
            max_index
        })
        .map(|i| (i + 'a' as usize) as u8 as char)
        .collect()
}

fn least_common_letters(codes: &[String]) -> String {
    count_letters(codes)
        .iter()
        .map(|count| {
            let mut min = usize::MAX;
            let mut min_index = 0;
            for (i, &c) in count.iter().enumerate() {
                if c > 0 && c < min {
                    min = c;
                    min_index = i;
                }
            }
            min_index
        })
        .map(|i| (i + 'a' as usize) as u8 as char)
        .collect()
}

fn count_letters(codes: &[String]) -> Vec<Vec<usize>> {
    codes
        .iter()
        .fold(vec![vec![0; 26]; codes[0].len()], |mut counts, code| {
            for (i, c) in code.chars().enumerate() {
                counts[i][c as usize - 'a' as usize] += 1;
            }
            counts
        })
}

pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

#[test]
fn test() {
    let test_input = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
";
    assert_eq!("easter", part1(&parse_input(test_input)));
    assert_eq!("advent", part2(&parse_input(test_input)));
}
