use super::intcode::Vm;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(vm: &Vm) -> i64 {
    let output = vm.clone().run(&walk_springdroid());
    *output.last().unwrap()
}

pub fn part2(vm: &Vm) -> i64 {
    let output = vm.clone().run(&run_springdroid());
    //println!("{}", output_to_string(&output));
    *output.last().unwrap()
}

//fn output_to_string(output: &[i64]) -> String {
//    output
//        .iter()
//        .filter_map(|&i| char::from_u32(i as u32))
//        .collect()
//}

// @
// ##..#    D has to be true or we'll die
//  ABCD    A,B or C should be a gap or there's no point jumping
//
// Jump if: D && !(A && B && C)
fn walk_springdroid() -> Vec<i64> {
    "\
OR A T
AND B T
AND C T
NOT T T
AND D T
OR T J
WALK
"
    .chars()
    .map(|c| c as i64)
    .collect()
}

// now there are 9 ground sensors A-I, we still jump the same distance (4)
//
// need to jump here:
//   @
// ###.#.#..###
//    ABCDEFGHI
//
// but not jump here:
//   @
// #####.#.#..#
//    ABCDEFGHI
//
// We now need to make sure that if there's somewhere to land and it won't
// lead to immediate death, ie, jump if:
//   D && H             land at d, then an immediate jump to H
//   or
//   D && E && I        land at d, 1 step, then jump to I
//   or
//   D && E && F        land at d, 2 steps, then jump and hope it's ok (it will be)
//
// We still need the same rules about a reason to jump: !(A && B && C)
fn run_springdroid() -> Vec<i64> {
    "\
NOT F J
NOT T J
OR I J
AND E J
OR H J
AND D J
NOT A T
NOT T T
AND B T
AND C T
NOT T T
AND T J
RUN
"
    .chars()
    .map(|c| c as i64)
    .collect()
}
