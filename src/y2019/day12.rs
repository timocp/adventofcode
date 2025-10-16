use std::cmp::Ordering;
use std::fmt;

pub struct Solver {
    moons: Vec<Moon>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            moons: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        let mut system = System::new(self.moons.clone());
        system.step(1000);
        system.total_energy().to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
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
struct Moon {
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
            .map(|ex| ex.split('=').last().unwrap().parse().unwrap())
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

fn parse_input(input: &str) -> Vec<Moon> {
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

    let mut system = System { steps: 0, moons };
    system.step(1);
    println!("{:?}", system);
    assert_eq!(2, system.moons[0].x);
    assert_eq!(-1, system.moons[0].y);
    assert_eq!(1, system.moons[0].z);
    assert_eq!(3, system.moons[0].dx);
    assert_eq!(-1, system.moons[0].dy);
    assert_eq!(-1, system.moons[0].dz);

    system.step(9);
    println!("{:?}", system);
    assert_eq!(179, system.total_energy())
}
