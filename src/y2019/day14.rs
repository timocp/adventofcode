use std::fmt;

pub struct Solver {
    recipes: Vec<Recipe>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            recipes: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        Nanofactory::new(&self.recipes)
            .ore_needed(FUEL, 1)
            .to_string()
    }

    fn part2(&self) -> String {
        Nanofactory::new(&self.recipes)
            .max_fuel(1000000000000)
            .to_string()
    }
}

struct Amount {
    quantity: u64,
    chemical: Sym,
}

impl fmt::Debug for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.quantity, sym_to_s(self.chemical))
    }
}

fn sym_to_s(s: Sym) -> String {
    format!(
        "{}{}{}{}{}",
        s[0] as char, s[1] as char, s[2] as char, s[3] as char, s[4] as char
    )
}

type Sym = [u8; 5];

const fn symbol(s: &str) -> Sym {
    let b = s.as_bytes();
    match b.len() {
        5.. => [b[0], b[1], b[2], b[3], b[4]],
        4 => [b[0], b[1], b[2], b[3], 32],
        3 => [b[0], b[1], b[2], 32, 32],
        2 => [b[0], b[1], 32, 32, 32],
        1 => [b[0], 32, 32, 32, 32],
        0 => [32, 32, 32, 32, 32],
    }
}

const ORE: Sym = symbol("ORE");
const FUEL: Sym = symbol("FUEL");

impl From<&str> for Amount {
    fn from(s: &str) -> Self {
        let mut i = s.split(' ');
        Self {
            quantity: i.next().unwrap().parse().unwrap(),
            chemical: symbol(i.next().unwrap()),
        }
    }
}

struct Recipe {
    inputs: Vec<Amount>,
    output: Amount,
}

impl From<&str> for Recipe {
    fn from(s: &str) -> Self {
        let mut line = s.split(" => ");
        let inputs = line.next().unwrap().split(", ").map(Amount::from).collect();
        Self {
            inputs,
            output: Amount::from(line.next().unwrap()),
        }
    }
}

struct Nanofactory<'a> {
    recipes: &'a [Recipe],
}

struct FactoryState {
    inventory: Vec<u64>, // index matches the output of the recipes
}

fn parse_input(input: &str) -> Vec<Recipe> {
    input.lines().map(Recipe::from).collect()
}

impl<'a> Nanofactory<'a> {
    fn new(recipes: &'a [Recipe]) -> Self {
        Self { recipes }
    }

    fn index_for(&self, chemical: Sym) -> usize {
        self.recipes
            .iter()
            .position(|r| r.output.chemical == chemical)
            .unwrap()
    }

    fn build(&self, chemical: Sym, num: u64, state: &mut FactoryState) -> u64 {
        let index = self.index_for(chemical);
        let recipe = &self.recipes[index];
        let repeats = num.div_ceil(recipe.output.quantity);
        let mut ore_used = 0;
        for input in recipe.inputs.iter() {
            let input_needed = input.quantity * repeats;
            if input.chemical == ORE {
                ore_used += input_needed;
            } else {
                let input_index = self.index_for(input.chemical);
                if state.inventory[input_index] < input_needed {
                    // need some more of this input chemical
                    ore_used += self.build(
                        input.chemical,
                        input_needed - state.inventory[input_index],
                        state,
                    );
                }
                state.inventory[input_index] -= input_needed;
            }
        }
        state.inventory[index] += repeats * recipe.output.quantity;
        ore_used
    }

    fn ore_needed(&self, chemical: Sym, num: u64) -> u64 {
        let mut state = FactoryState {
            inventory: vec![0; self.recipes.len()],
        };
        self.build(chemical, num, &mut state)
    }

    // search for the most amount of fuel that will use < the maximum amount of ore
    fn max_fuel(&self, max_ore: u64) -> u64 {
        let f1 = self.ore_needed(FUEL, 1); // amount of ore to build 1 FUEL

        let mut range = 0..max_ore; // answer is somewhere in here
        loop {
            let guess = range.start + (range.end - range.start) / 2;
            let ore = self.ore_needed(FUEL, guess);
            if ore > max_ore {
                range.end = guess
            } else if ore <= max_ore - f1 {
                range.start = guess + 1;
            } else {
                return guess;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let r = parse_input(EXAMPLE1);
        let n = Nanofactory::new(&r);
        assert_eq!(10, n.recipes[0].inputs[0].quantity);
        assert_eq!(symbol("ORE"), n.recipes[0].inputs[0].chemical);
        assert_eq!(10, n.recipes[0].output.quantity);
        assert_eq!(symbol("A"), n.recipes[0].output.chemical);
    }

    #[test]
    fn test_ore_needed() {
        let example1 = parse_input(EXAMPLE1);
        let example2 = parse_input(EXAMPLE2);
        let example3 = parse_input(EXAMPLE3);
        let example4 = parse_input(EXAMPLE4);
        let example5 = parse_input(EXAMPLE5);
        assert_eq!(31, Nanofactory::new(&example1).ore_needed(FUEL, 1));
        assert_eq!(165, Nanofactory::new(&example2).ore_needed(FUEL, 1));
        assert_eq!(13312, Nanofactory::new(&example3).ore_needed(FUEL, 1));
        assert_eq!(180697, Nanofactory::new(&example4).ore_needed(FUEL, 1));
        assert_eq!(2210736, Nanofactory::new(&example5).ore_needed(FUEL, 1));
    }

    #[test]
    fn test_max_fuel() {
        let example3 = parse_input(EXAMPLE3);
        let example4 = parse_input(EXAMPLE4);
        let example5 = parse_input(EXAMPLE5);
        assert_eq!(
            82892753,
            Nanofactory::new(&example3).max_fuel(1000000000000)
        );
        assert_eq!(5586022, Nanofactory::new(&example4).max_fuel(1000000000000));
        assert_eq!(460664, Nanofactory::new(&example5).max_fuel(1000000000000));
    }

    const EXAMPLE1: &str = "\
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
";

    const EXAMPLE2: &str = "\
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL
";

    const EXAMPLE3: &str = "\
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
";

    const EXAMPLE4: &str = "\
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF
";

    const EXAMPLE5: &str = "\
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX
";
}
