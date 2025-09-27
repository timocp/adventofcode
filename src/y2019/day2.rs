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
        for noun in 0..100 {
            for verb in 0..100 {
                let mut vm = self.vm.clone();
                vm.write(1, noun);
                vm.write(2, verb);
                vm.run();
                if vm.read(0) == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        panic!("No solution found");
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
