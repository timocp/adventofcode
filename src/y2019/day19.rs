use super::intcode::Vm;

pub struct Solver {
    vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let mut count = 0;
        for x in 0..50 {
            for y in 0..50 {
                count += self.vm.clone().run(&[x, y]).first().unwrap();
            }
        }
        count.to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}
