use super::intcode::Vm;

pub struct Solver {
    nic_vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            nic_vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let mut network: Vec<Computer> = (0..50)
            .map(|id| {
                let mut vm = self.nic_vm.clone();
                vm.set_input(&[id]);
                Computer {
                    vm,
                    queue: Vec::new(),
                }
            })
            .collect();

        loop {
            for n in 0..50 {
                let output = if network[n].queue.is_empty() {
                    network[n].vm.run(&[-1])
                } else {
                    let input = &network[n].queue.to_owned();
                    network[n].vm.run(input)
                };
                for packet in output.chunks(3) {
                    let destination = packet[0] as usize;
                    if destination == 255 {
                        return packet[2].to_string();
                    }
                    network[destination].queue.push(packet[1]); // X
                    network[destination].queue.push(packet[2]); // Y
                }
            }
        }
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

struct Computer {
    vm: Vm,
    queue: Vec<i64>,
}
