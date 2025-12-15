use std::collections::HashSet;

use super::intcode::Vm;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(nic_vm: &Vm) -> i64 {
    run_network(nic_vm, false)
}

pub fn part2(nic_vm: &Vm) -> i64 {
    run_network(nic_vm, true)
}

struct Computer {
    vm: Vm,
    queue: Vec<i64>,
}

fn run_network(nic_vm: &Vm, full: bool) -> i64 {
    let mut network: Vec<Computer> = (0..50)
        .map(|id| {
            let mut vm = nic_vm.clone();
            vm.set_input(&[id]);
            Computer {
                vm,
                queue: Vec::new(),
            }
        })
        .collect();

    // Y parts of packets that have been sent from NAT to device 0
    let mut seen: HashSet<i64> = HashSet::new();

    loop {
        let mut idle = false;
        let mut nat_packet: Option<(i64, i64)> = None;

        while !idle {
            idle = true;
            for n in 0..50 {
                let output = if network[n].queue.is_empty() {
                    network[n].vm.run(&[-1])
                } else {
                    idle = false;
                    let input = &network[n].queue.to_owned();
                    network[n].queue.clear();
                    network[n].vm.run(input)
                };
                for packet in output.chunks(3) {
                    idle = false;
                    let destination = packet[0] as usize;
                    if destination == 255 {
                        if full {
                            // part 2 - NAT retains last packet received
                            nat_packet = Some((packet[1], packet[2]));
                        } else {
                            // part 1 - Y of first packet received by NAT
                            return packet[2];
                        }
                    } else {
                        network[destination].queue.push(packet[1]); // X
                        network[destination].queue.push(packet[2]); // Y
                    }
                }
            }
        }

        // network is idle, the last packet sent to device 255 should be sent to computer 0
        let nat_packet = nat_packet.unwrap();
        if seen.contains(&nat_packet.1) {
            return nat_packet.1;
        }
        seen.insert(nat_packet.1);
        network[0].queue.push(nat_packet.0);
        network[0].queue.push(nat_packet.1);
    }
}
