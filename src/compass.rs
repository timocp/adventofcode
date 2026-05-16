// Compass directions
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const ALL_DIRS: [Compass; 8] = [
    Compass::North,
    Compass::NorthEast,
    Compass::East,
    Compass::SouthEast,
    Compass::South,
    Compass::SouthWest,
    Compass::West,
    Compass::NorthWest,
];

impl Compass {
    pub fn left90(&self) -> Self {
        match self {
            Compass::North => Compass::West,
            Compass::NorthEast => Compass::NorthWest,
            Compass::East => Compass::North,
            Compass::SouthEast => Compass::NorthEast,
            Compass::South => Compass::East,
            Compass::SouthWest => Compass::SouthEast,
            Compass::West => Compass::South,
            Compass::NorthWest => Compass::SouthWest,
        }
    }

    pub fn right90(&self) -> Self {
        match self {
            Compass::North => Compass::East,
            Compass::NorthEast => Compass::SouthEast,
            Compass::East => Compass::South,
            Compass::SouthEast => Compass::SouthWest,
            Compass::South => Compass::West,
            Compass::SouthWest => Compass::NorthWest,
            Compass::West => Compass::North,
            Compass::NorthWest => Compass::NorthEast,
        }
    }
}

