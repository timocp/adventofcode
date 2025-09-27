#[derive(Debug, Clone)]
pub struct Vm {
    intcode: Vec<i32>,
    pc: usize,
}

impl Vm {
    fn exec(&mut self) {
        match self.intcode[self.pc] {
            1 => {
                self.indirect_write(
                    self.pc + 3,
                    self.indirect_read(self.pc + 1) + self.indirect_read(self.pc + 2),
                );
                self.pc += 4;
            }
            2 => {
                self.indirect_write(
                    self.pc + 3,
                    self.indirect_read(self.pc + 1) * self.indirect_read(self.pc + 2),
                );
                self.pc += 4;
            }
            99 => {}
            _ => panic!(
                "invalid instruction: {} at {}",
                self.intcode[self.pc], self.pc
            ),
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted() {
            self.exec();
        }
    }

    pub fn write(&mut self, pos: usize, value: i32) {
        self.intcode[pos] = value;
    }

    pub fn read(&self, pos: usize) -> i32 {
        self.intcode[pos]
    }

    fn indirect_read(&self, pos: usize) -> i32 {
        //let index = self.intcode[pos] as usize;
        //self.intcode[index]
        self.read(self.intcode[pos] as usize)
    }

    fn indirect_write(&mut self, pos: usize, value: i32) {
        //let index = self.intcode[pos] as usize;
        //self.intcode[index] = value;
        self.write(self.intcode[pos] as usize, value);
    }

    fn is_halted(&self) -> bool {
        self.intcode[self.pc] == 99
    }
}

impl From<&str> for Vm {
    fn from(s: &str) -> Self {
        Vm {
            intcode: s
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect(),
            pc: 0,
        }
    }
}

#[test]
fn test_from() {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let vm = Vm::from(test_input);
    assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vm.intcode);
    assert_eq!(0, vm.pc);
}

#[test]
fn test_add() {
    let mut vm = Vm {
        intcode: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        pc: 0,
    };
    assert_eq!(3, vm.intcode[3]);
    vm.exec();
    assert_eq!(70, vm.intcode[3]);
    assert_eq!(4, vm.pc);
}

#[test]
fn test_mul() {
    let mut vm = Vm {
        intcode: vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        pc: 4,
    };
    assert_eq!(1, vm.intcode[0]);
    vm.exec();
    assert_eq!(3500, vm.intcode[0]);
    assert_eq!(8, vm.pc);
}

#[test]
fn test_is_halted() {
    let vm = Vm {
        intcode: vec![99],
        pc: 0,
    };
    assert!(vm.is_halted());
}

#[test]
fn test_run() {
    let mut vm = Vm::from("1,0,0,0,99");
    vm.run();
    assert_eq!(vec![2, 0, 0, 0, 99], vm.intcode);

    let mut vm = Vm::from("2,3,0,3,99");
    vm.run();
    assert_eq!(vec![2, 3, 0, 6, 99], vm.intcode);

    let mut vm = Vm::from("2,4,4,5,99,0");
    vm.run();
    assert_eq!(vec![2, 4, 4, 5, 99, 9801], vm.intcode);

    let mut vm = Vm::from("1,1,1,4,99,5,6,0,99");
    vm.run();
    assert_eq!(vec![30, 1, 1, 4, 2, 5, 6, 0, 99], vm.intcode);
}
