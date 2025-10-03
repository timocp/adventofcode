use super::intcode::Vm;

pub struct Solver {
    boost_vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            boost_vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let output = self.boost_vm.clone().run(&[1]);
        if output.len() != 1 {
            panic!("BOOST error: {:?}", output);
        }
        output[0].to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}
