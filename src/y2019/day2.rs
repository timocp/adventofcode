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
        let mut vm = self.vm.clone();
        vm.write(1, 12);
        vm.write(2, 2);
        vm.run();
        vm.read(0).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

#[test]
fn test() {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut vm = Vm::from(test_input);
    vm.run();
    assert_eq!(3500, vm.read(0));
    assert_eq!(70, vm.read(3));
}
