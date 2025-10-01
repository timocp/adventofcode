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
        "unimplemented".to_string()
    }
}

fn count_pixels(layer: &[u8], p: u8) -> usize {
    layer.iter().filter(|&&v| v == p).count()
}

#[derive(Debug)]
struct Image {
    width: u32,
    height: u32,
    bytes: Vec<u8>,
}

impl Image {
    fn new(s: &str, width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            bytes: s.trim().as_bytes().iter().map(|&b| b - b'0').collect(),
        }
    }

    fn each_layer(&self) -> impl Iterator<Item = &[u8]> {
        self.bytes.chunks((self.width * self.height) as usize)
    }
}

#[test]
fn test_parse() {
    let image = Image::new("123456789012\n", 3, 2);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2], image.bytes);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], image.each_layer().next().unwrap());
}
