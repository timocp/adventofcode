use crate::Puzzle;

pub struct Solver {
    input: Vec<Room>,
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    // sum of the sector IDs of the real rooms
    fn part1(&self) -> String {
        self.input
            .iter()
            .filter(|room| room.is_real())
            .map(|room| room.sector_id)
            .sum::<u32>()
            .to_string()
    }

    // sector ID of the room where north pole objects are stored
    fn part2(&self) -> String {
        self.input
            .iter()
            .filter(|room| room.is_real())
            .find(|room| room.name() == "northpole object storage")
            .unwrap()
            .sector_id
            .to_string()
    }
}

fn parse_input(input: &str) -> Vec<Room> {
    input.lines().map(|line| line.into()).collect()
}

struct Room {
    encrypted_name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn is_real(&self) -> bool {
        self.checksum == self.calc_checksum()
    }

    fn calc_checksum(&self) -> String {
        let mut counts = self
            .encrypted_name
            .chars()
            .filter(|&c| c.is_ascii_lowercase())
            .fold([0; 26], |mut counts, c| {
                counts[c as usize - 'a' as usize] += 1;
                counts
            });
        let mut cs = String::with_capacity(5);
        for _ in 0..5 {
            let mut max = 0;
            let mut letter = 0;
            for (i, count) in counts.iter().enumerate() {
                if *count > max {
                    max = *count;
                    letter = i;
                }
            }
            cs.push((letter as u8 + b'a') as char);
            counts[letter] = 0;
        }
        cs
    }

    fn name(&self) -> String {
        self.encrypted_name
            .chars()
            .map(|c| match c {
                '-' => ' ',
                'a'..='z' => {
                    char::from_u32((((c as u32) - 97 + self.sector_id) % 26) + 97).unwrap()
                }
                _ => panic!("invalid character in room name: {}", c),
            })
            .collect()
    }
}

impl From<&str> for Room {
    fn from(s: &str) -> Self {
        let dash = s.rfind('-').unwrap();
        let bracket1 = s.rfind('[').unwrap();
        let bracket2 = s.rfind(']').unwrap();
        Self {
            encrypted_name: s[..dash].to_string(),
            sector_id: s[dash + 1..bracket1].parse().unwrap(),
            checksum: s[bracket1 + 1..bracket2].to_string(),
        }
    }
}

#[test]
fn test() {
    let rooms = [
        "aaaaa-bbb-z-y-x-123[abxyz]",
        "a-b-c-d-e-f-g-h-987[abcde]",
        "not-a-real-room-404[oarel]",
        "totally-real-room-200[decoy]",
    ]
    .map(|input| Room::from(input));

    assert_eq!(rooms[0].encrypted_name, "aaaaa-bbb-z-y-x");
    assert_eq!(rooms[0].sector_id, 123);
    assert_eq!(rooms[0].checksum, "abxyz");
    assert!(rooms[0].is_real());

    assert_eq!(rooms[1].encrypted_name, "a-b-c-d-e-f-g-h");
    assert_eq!(rooms[1].sector_id, 987);
    assert_eq!(rooms[1].checksum, "abcde");
    assert!(rooms[1].is_real());

    assert_eq!(rooms[2].encrypted_name, "not-a-real-room");
    assert_eq!(rooms[2].sector_id, 404);
    assert_eq!(rooms[2].checksum, "oarel");
    assert!(rooms[2].is_real());

    assert_eq!(rooms[3].encrypted_name, "totally-real-room");
    assert_eq!(rooms[3].sector_id, 200);
    assert_eq!(rooms[3].checksum, "decoy");
    assert!(!rooms[3].is_real());

    assert_eq!(
        "very encrypted name",
        Room::from("qzmt-zixmtkozy-ivhz-343[xxxxx]").name()
    );
}
