use std::fmt;

use crate::pixel_buffer::PixelBuffer;

pub struct Solver {
    image: Image,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            image: Image::new(input, 25, 6),
        }
    }

    fn part1(&self) -> String {
        // find layer with the fewest 0s
        let layer = self
            .image
            .each_layer()
            .min_by_key(|&layer| count_pixels(layer, 0))
            .unwrap();

        (count_pixels(layer, 1) * count_pixels(layer, 2)).to_string()
    }

    fn part2(&self) -> String {
        self.image.to_string()
    }
}

fn count_pixels(layer: &[u8], p: u8) -> usize {
    layer.iter().filter(|&&v| v == p).count()
}

#[derive(Debug)]
struct Image {
    width: u32,
    height: u32,
    area: u32,
    bytes: Vec<u8>,
}

impl Image {
    fn new(s: &str, width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            area: width * height,
            bytes: s.trim().as_bytes().iter().map(|&b| b - b'0').collect(),
        }
    }

    fn each_layer(&self) -> impl Iterator<Item = &[u8]> {
        self.bytes.chunks((self.width * self.height) as usize)
    }

    fn get(&self, x: u32, y: u32, layer: u32) -> u8 {
        self.bytes[(x + y * self.width + layer * self.area) as usize]
    }

    fn is_lit(&self, x: u32, y: u32) -> bool {
        for layer in 0..(self.bytes.len() as u32 / self.area) {
            match self.get(x, y, layer) {
                0 => return false,
                1 => return true,
                _ => {}
            };
        }
        panic!();
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = PixelBuffer::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                buffer.set(x, y, self.is_lit(x, y));
            }
        }
        f.write_str(&buffer.to_string())
    }
}

#[test]
fn test_parse() {
    let image = Image::new("123456789012\n", 3, 2);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], image.bytes);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], image.each_layer().next().unwrap());
}

#[test]
fn test_decode() {
    let image = Image::new("0222112222120000", 2, 2);
    assert_eq!("▞", image.to_string());
}
