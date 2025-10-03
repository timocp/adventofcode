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
        self.get_result(12, 2).to_string()
    }

    fn part2(&self) -> String {
        for noun in 0..100 {
            for verb in 0..100 {
                if self.get_result(noun, verb) == 19690720 {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        panic!("No solution found");
    }
}

impl Solver {
    // Return the final value at address 0 if the program is run to completion
    fn get_result(&self, noun: i64, verb: i64) -> i64 {
        let mut vm = self.vm.clone();
        vm.direct_write(1, noun);
        vm.direct_write(2, verb);
        vm.exec();
        vm.direct_read(0)
    }
}

#[test]
fn test() {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut vm = Vm::from(test_input);
    vm.exec();
    assert_eq!(3500, vm.direct_read(0));
    assert_eq!(70, vm.direct_read(3));
}
