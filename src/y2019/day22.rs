pub struct Solver {
    techniques: Vec<Technique>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            techniques: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        shuffle((0..10007).collect(), &self.techniques)
            .iter()
            .position(|n| *n == 2019)
            .unwrap()
            .to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn shuffle(cards: Vec<i32>, techniques: &[Technique]) -> Vec<i32> {
    techniques.iter().fold(cards, |acc, t| shuffle1(&acc, t))
}

fn shuffle1(cards: &[i32], technique: &Technique) -> Vec<i32> {
    match technique {
        Technique::Cut(at) => cut(cards, *at),
        Technique::DealIntoNewStack => deal_into_new_stack(cards),
        Technique::DealWithIncrement(inc) => deal_with_increment(cards, *inc),
    }
}

#[derive(Debug)]
enum Technique {
    Cut(i32),
    DealIntoNewStack,
    DealWithIncrement(usize),
}

fn deal_into_new_stack(cards: &[i32]) -> Vec<i32> {
    (0..cards.len())
        .map(|i| cards[cards.len() - 1 - i])
        .collect()
}

fn cut_front(cards: &[i32], at: usize) -> Vec<i32> {
    (0..cards.len())
        .map(|i| {
            if i < cards.len() - at {
                cards[i + at]
            } else {
                cards[i - (cards.len() - at)]
            }
        })
        .collect()
}

fn cut(cards: &[i32], at: i32) -> Vec<i32> {
    if at < 0 {
        cut_front(cards, cards.len() - at.unsigned_abs() as usize)
    } else {
        cut_front(cards, at as usize)
    }
}

fn deal_with_increment(cards: &[i32], inc: usize) -> Vec<i32> {
    let mut new: Vec<i32> = vec![0; cards.len()];
    let mut pos = 0;
    for i in 0..cards.len() {
        new[pos] = cards[i];
        pos = (pos + inc) % cards.len();
    }
    new
}

impl From<&str> for Technique {
    fn from(s: &str) -> Self {
        if let Some(number) = s.strip_prefix("cut ") {
            Technique::Cut(number.parse().unwrap())
        } else if s == "deal into new stack" {
            Technique::DealIntoNewStack
        } else if let Some(number) = s.strip_prefix("deal with increment ") {
            Technique::DealWithIncrement(number.parse().unwrap())
        } else {
            panic!("invalid technique: {}", s)
        }
    }
}

fn parse_input(input: &str) -> Vec<Technique> {
    input.lines().map(|line| line.into()).collect()
}

#[test]
fn test_techniques() {
    let deck10: Vec<i32> = (0..10).collect();

    assert_eq!(
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
        deal_into_new_stack(&deck10)
    );

    assert_eq!(vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2], cut(&deck10, 3));
    assert_eq!(vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5], cut(&deck10, -4));

    assert_eq!(
        vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3],
        deal_with_increment(&deck10, 3)
    );
}

#[test]
fn test_shuffle() {
    assert_eq!(
        vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7],
        shuffle(
            (0..10).collect(),
            &vec![
                Technique::DealWithIncrement(7),
                Technique::DealIntoNewStack,
                Technique::DealIntoNewStack,
            ]
        )
    );

    assert_eq!(
        vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6],
        shuffle(
            (0..10).collect(),
            &vec![
                Technique::Cut(6),
                Technique::DealWithIncrement(7),
                Technique::DealIntoNewStack,
            ]
        )
    );

    assert_eq!(
        vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9],
        shuffle(
            (0..10).collect(),
            &vec![
                Technique::DealWithIncrement(7),
                Technique::DealWithIncrement(9),
                Technique::Cut(-2),
            ]
        )
    );

    assert_eq!(
        vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6],
        shuffle(
            (0..10).collect(),
            &vec![
                Technique::DealIntoNewStack,
                Technique::Cut(-2),
                Technique::DealWithIncrement(7),
                Technique::Cut(8),
                Technique::Cut(-4),
                Technique::DealWithIncrement(7),
                Technique::Cut(3),
                Technique::DealWithIncrement(9),
                Technique::DealWithIncrement(3),
                Technique::Cut(-1),
            ]
        )
    );
}
