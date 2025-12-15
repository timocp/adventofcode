use num_integer::lcm;
use std::cmp::Ordering;
use std::fmt;

pub fn part1(moons: &[Moon]) -> u16 {
    let mut system = System::new(moons.to_owned());
    system.step(1000);
    system.total_energy()
}

pub fn part2(moons: &[Moon]) -> u64 {
    let mut system = System::new(moons.to_owned());
    system.count_to_repeat()
}

struct System {
    steps: u32,
    moons: Vec<Moon>,
}

impl System {
    fn new(moons: Vec<Moon>) -> System {
        System { steps: 0, moons }
    }

    fn step(&mut self, count: u32) {
        for _ in 0..count {
            self.apply_gravity();
        }
        self.steps += count;
    }

    fn apply_gravity(&mut self) {
        for a in 0..self.moons.len() {
            for b in (a + 1)..self.moons.len() {
                let (gx, gy, gz) = self.moons[a].calc_gravity(&self.moons[b]);
                self.moons[a].dx -= gx;
                self.moons[b].dx += gx;
                self.moons[a].dy -= gy;
                self.moons[b].dy += gy;
                self.moons[a].dz -= gz;
                self.moons[b].dz += gz;
            }
        }
        self.moons.iter_mut().for_each(|moon| moon.step());
    }

    fn total_energy(&self) -> u16 {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }

    fn count_to_repeat(&mut self) -> u64 {
        // step until we see a repeated state in x, y and z axis individually
        let start = [self.state_x(), self.state_y(), self.state_z()];
        let mut repeats: [Option<u64>; 3] = [None, None, None];
        while repeats[0].is_none() || repeats[1].is_none() || repeats[2].is_none() {
            self.step(1);
            if repeats[0].is_none() && start[0] == self.state_x() {
                repeats[0] = Some(self.steps.into());
            }
            if repeats[1].is_none() && start[1] == self.state_y() {
                repeats[1] = Some(self.steps.into());
            }
            if repeats[2].is_none() && start[2] == self.state_z() {
                repeats[2] = Some(self.steps.into());
            }
        }
        // the entire system repeats at the least common multiple of each axis
        lcm(
            repeats[0].unwrap(),
            lcm(repeats[1].unwrap(), repeats[2].unwrap()),
        )
    }

    fn state_x(&self) -> Vec<i16> {
        self.moons
            .iter()
            .flat_map(|moon| [moon.x, moon.dx])
            .collect()
    }

    fn state_y(&self) -> Vec<i16> {
        self.moons
            .iter()
            .flat_map(|moon| [moon.y, moon.dy])
            .collect()
    }

    fn state_z(&self) -> Vec<i16> {
        self.moons
            .iter()
            .flat_map(|moon| [moon.z, moon.dz])
            .collect()
    }
}

impl fmt::Debug for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "After {} steps:", self.steps)?;
        for moon in self.moons.iter() {
            writeln!(
                f,
                "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
                moon.x, moon.y, moon.z, moon.dx, moon.dy, moon.dz
            )?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Moon {
    // position
    x: i16,
    y: i16,
    z: i16,
    //velocity
    dx: i16,
    dy: i16,
    dz: i16,
}

impl Moon {
    fn calc_gravity(&self, other: &Moon) -> (i16, i16, i16) {
        (
            compare(self.x, other.x),
            compare(self.y, other.y),
            compare(self.z, other.z),
        )
    }

    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    fn total_energy(&self) -> u16 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> u16 {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }

    fn kinetic_energy(&self) -> u16 {
        self.dx.unsigned_abs() + self.dy.unsigned_abs() + self.dz.unsigned_abs()
    }
}

// returns -1, 0 or 1, like <=> operator
fn compare(a: i16, b: i16) -> i16 {
    match a.cmp(&b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

impl From<&str> for Moon {
    fn from(s: &str) -> Self {
        let s = &s[1..(s.len() - 1)];
        let coords: Vec<_> = s
            .split(", ")
            .map(|ex| ex.split('=').next_back().unwrap().parse().unwrap())
            .collect();
        Moon {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            dx: 0,
            dy: 0,
            dz: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Moon> {
    input.lines().map(Moon::from).collect()
}

#[test]
fn test_moon_system() {
    let test_input = "\
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";
    let moons = parse_input(test_input);
    assert_eq!(-1, moons[0].x);
    assert_eq!(0, moons[0].y);
    assert_eq!(2, moons[0].z);
    assert_eq!(0, moons[0].dx);
    assert_eq!(0, moons[0].dy);
    assert_eq!(0, moons[0].dz);

    let mut system = System::new(moons);
    system.step(1);
    assert_eq!(2, system.moons[0].x);
    assert_eq!(-1, system.moons[0].y);
    assert_eq!(1, system.moons[0].z);
    assert_eq!(3, system.moons[0].dx);
    assert_eq!(-1, system.moons[0].dy);
    assert_eq!(-1, system.moons[0].dz);

    system.step(9);
    assert_eq!(179, system.total_energy())
}

#[test]
fn test_count_to_repeat() {
    let test_input = "\
<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>
";
    let mut system = System::new(parse_input(test_input));
    assert_eq!(4686774924, system.count_to_repeat());
}
