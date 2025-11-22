use std::fmt;

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
        compose_techniques(&self.techniques, 10007)
            .apply(2019)
            .to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

#[derive(Debug)]
enum Technique {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

// map an array of shuffling techniques into LCF functions,
// and then compose them into a single function.
fn compose_techniques(techniques: &[Technique], m: i128) -> Lcf {
    techniques
        .iter()
        .map(|t| t.lcf(m))
        .reduce(|f, g| f.compose(&g))
        .unwrap()
}

// Represents the linear congruence function f(x) = ax + b mod m
struct Lcf {
    a: i128,
    b: i128,
    m: i128,
}

impl Lcf {
    // based on guide: https://codeforces.com/blog/entry/72593
    // f(x) = ax + b mod m  (self)
    // g(x) = cx + d mod m  (other)
    // g(f(x)) = c(ax + b) + d mod m
    //         = cax + cb + d mod m
    fn compose(&self, other: &Self) -> Self {
        if self.m != other.m {
            unreachable!();
        }
        let a = self.a;
        let b = self.b;
        let c = other.a;
        let d = other.b;
        Lcf {
            a: (c * a).rem_euclid(self.m),
            b: (c * b + d).rem_euclid(self.m),
            m: self.m,
        }
    }

    fn apply(&self, x: i128) -> i128 {
        (self.a * x + self.b).rem_euclid(self.m)
    }
}

impl fmt::Debug for Lcf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} * x {} {} mod {}",
            self.a,
            if self.b.signum() < 0 { '-' } else { '+' },
            self.b.abs(),
            self.m
        )
    }
}

impl Technique {
    fn lcf(&self, m: i128) -> Lcf {
        match self {
            Technique::DealIntoNewStack => Lcf { a: -1, b: -1, m },
            Technique::Cut(n) => Lcf { a: 1, b: -n, m },
            Technique::DealWithIncrement(n) => Lcf { a: *n, b: 0, m },
        }
    }
}

impl From<&str> for Technique {
    fn from(s: &str) -> Self {
        if s == "deal into new stack" {
            Technique::DealIntoNewStack
        } else if let Some(number) = s.strip_prefix("cut ") {
            Technique::Cut(number.parse().unwrap())
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
fn test_apply() {
    let f = Technique::DealIntoNewStack.lcf(10);

    for (index, x) in vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x as i128));
    }

    let f = Technique::Cut(3).lcf(10);
    for (index, x) in vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }

    let f = Technique::Cut(-4).lcf(10);
    for (index, x) in vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }

    let f = Technique::DealWithIncrement(3).lcf(10);
    for (index, x) in vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }
}

#[test]
fn test_compose() {
    let f = compose_techniques(
        &vec![
            Technique::DealWithIncrement(7),
            Technique::DealIntoNewStack,
            Technique::DealIntoNewStack,
        ],
        10,
    );
    for (index, x) in vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }

    let f = compose_techniques(
        &vec![
            Technique::Cut(6),
            Technique::DealWithIncrement(7),
            Technique::DealIntoNewStack,
        ],
        10,
    );
    for (index, x) in vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }

    let f = compose_techniques(
        &vec![
            Technique::DealWithIncrement(7),
            Technique::DealWithIncrement(9),
            Technique::Cut(-2),
        ],
        10,
    );
    for (index, x) in vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }

    let f = compose_techniques(
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
        ],
        10,
    );
    for (index, x) in vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6].into_iter().enumerate() {
        assert_eq!(index as i128, f.apply(x));
    }
}
