use std::fmt;

// screen/buffer of pixels.  (0,0) is top left.
// outputs using unicode block elements mapping 4 pixels per character
//
// could probably replace 2016 day 8 Screen with light/rotate fns
pub struct PixelBuffer {
    width: u32,
    height: u32,
    bits: Vec<bool>,
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            bits: vec![false; (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, v: bool) {
        if x >= self.width {
            panic!("set: out of bounds x {} >= {}", x, self.width);
        }
        if y >= self.height {
            panic!("set: out of bounds y {} >= {}", y, self.height);
        }

        self.bits[(x + y * self.width) as usize] = v
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.width {
            false
        } else {
            self.bits[(x + y * self.width) as usize]
        }
    }
}

const TL: u32 = 8;
const TR: u32 = 4;
const BL: u32 = 2;
const BR: u32 = 1;

impl fmt::Display for PixelBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..self.height).step_by(2) {
            if y > 0 {
                writeln!(f)?;
            }
            for x in (0..self.width).step_by(2) {
                write!(
                    f,
                    "{}",
                    match self.pattern(x, y) {
                        0b1111 => '█',
                        0b1110 => '▛',
                        0b1101 => '▜',
                        0b1100 => '▀',
                        0b1011 => '▙',
                        0b1010 => '▌',
                        0b1001 => '▚',
                        0b1000 => '▘',
                        0b0111 => '▟',
                        0b0110 => '▞',
                        0b0101 => '▐',
                        0b0100 => '▝',
                        0b0011 => '▄',
                        0b0010 => '▖',
                        0b0001 => '▗',
                        0b0000 => ' ',
                        _ => panic!(),
                    }
                )?;
            }
        }
        Ok(())
    }
}

impl PixelBuffer {
    fn pattern(&self, x: u32, y: u32) -> u32 {
        let tl = if self.get(x, y) { TL } else { 0 };
        let tr = if self.get(x + 1, y) { TR } else { 0 };
        let bl = if self.get(x, y + 1) { BL } else { 0 };
        let br = if self.get(x + 1, y + 1) { BR } else { 0 };
        tl + tr + bl + br
    }
}
