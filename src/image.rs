use crate::pos::Pos;
use std::fs::File;
use std::io::BufWriter;
use std::ops::Range;
use std::path::Path;

// Wrapper around png to generate 8-color pngs for visualisation or debugging
pub struct Image {
    x_offset: i32,
    y_offset: i32,
    width: usize,
    height: usize,
    data: Vec<u8>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Colour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Yellow,
    White,
}

impl Colour {
    fn to_bytes(&self) -> &'static [u8] {
        match self {
            Self::Black => &[0, 0, 0],
            Self::Blue => &[0, 0, 255],
            Self::Green => &[0, 255, 0],
            Self::Cyan => &[0, 255, 255],
            Self::Red => &[255, 0, 0],
            Self::Magenta => &[255, 0, 255],
            Self::Yellow => &[255, 255, 0],
            Self::White => &[255, 255, 255],
        }
    }
}

impl Image {
    pub fn new(topleft: &Pos, bottomright: &Pos) -> Self {
        let x_offset = topleft.x;
        let y_offset = topleft.y;
        let width: usize = (bottomright.x - x_offset + 1).try_into().unwrap();
        let height: usize = (bottomright.y - y_offset + 1).try_into().unwrap();

        Self {
            x_offset,
            y_offset,
            width,
            height,
            data: vec![255; width * height * 3],
        }
    }

    pub fn set(&mut self, p: &Pos, colour: Colour) {
        let range = self.range_for(p);
        self.data[range].copy_from_slice(colour.to_bytes());
    }

    fn range_for(&self, p: &Pos) -> Range<usize> {
        let x: usize = (p.x - self.x_offset).try_into().unwrap();
        let y: usize = (p.y - self.y_offset).try_into().unwrap();
        let offset = (y * self.width + x) * 3;
        offset..(offset + 3)
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        print!("Writing {filename}... ");
        let path = Path::new(filename);
        let file = File::create(path)?;
        let writer = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&self.data)?;
        println!("Done.");
        Ok(())
    }
}
