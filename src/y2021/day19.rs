use crate::pos::{ORIGIN3D, Pos3d};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

pub struct Input {
    beacons: HashSet<Pos3d>,
    scanners: Vec<Pos3d>,
}

pub fn parse_input(input: &str) -> Input {
    let scans = parse_scans(input);
    let (beacons, scanners) = search(&scans);
    Input { beacons, scanners }
}

pub fn part1(input: &Input) -> usize {
    input.beacons.len()
}

pub fn part2(input: &Input) -> u32 {
    max_distance(&input.scanners)
}

impl Pos3d {
    // each possible rotation from: http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
    fn rotate(&self, rot: i32) -> Pos3d {
        match rot {
            0 => Pos3d::from((self.x, self.y, self.z)),
            1 => Pos3d::from((self.x, self.z, -self.y)),
            2 => Pos3d::from((self.x, -self.y, -self.z)),
            3 => Pos3d::from((self.x, -self.z, self.y)),
            4 => Pos3d::from((self.y, -self.x, self.z)),
            5 => Pos3d::from((self.y, self.z, self.x)),
            6 => Pos3d::from((self.y, self.x, -self.z)),
            7 => Pos3d::from((self.y, -self.z, -self.x)),
            8 => Pos3d::from((-self.x, -self.y, self.z)),
            9 => Pos3d::from((-self.x, -self.z, -self.y)),
            10 => Pos3d::from((-self.x, self.y, -self.z)), // scan0 -> scan1 from example
            11 => Pos3d::from((-self.x, self.z, self.y)),
            12 => Pos3d::from((-self.y, self.x, self.z)),
            13 => Pos3d::from((-self.y, -self.z, self.x)),
            14 => Pos3d::from((-self.y, -self.x, -self.z)),
            15 => Pos3d::from((-self.y, self.z, -self.x)),
            16 => Pos3d::from((self.z, self.y, -self.x)),
            17 => Pos3d::from((self.z, self.x, self.y)),
            18 => Pos3d::from((self.z, -self.y, self.x)),
            19 => Pos3d::from((self.z, -self.x, -self.y)),
            20 => Pos3d::from((-self.z, -self.y, -self.x)),
            21 => Pos3d::from((-self.z, -self.x, self.y)),
            22 => Pos3d::from((-self.z, self.y, self.x)),
            23 => Pos3d::from((-self.z, self.x, -self.y)),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
struct Scan {
    number: usize,
    beacons: Vec<Pos3d>,
}

impl fmt::Display for Scan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "scanner {} (sees {} beacons)",
            self.number,
            self.beacons.len()
        )
    }
}

impl From<&str> for Scan {
    fn from(s: &str) -> Self {
        let mut beacons = vec![];
        let mut number: usize = 0;

        for line in s.lines() {
            if line.starts_with("--- scanner ") {
                number = line
                    .chars()
                    .skip(12)
                    .take_while(|c| !c.is_ascii_whitespace())
                    .collect::<String>()
                    .parse()
                    .unwrap();
            } else {
                beacons.push(line.parse().unwrap());
            }
        }

        Self { number, beacons }
    }
}

fn parse_scans(input: &str) -> Vec<Scan> {
    input.split("\n\n").map(Scan::from).collect()
}

// for storing pre-rotated sets of beacons
struct RotatedBeacons {
    scan_number: usize,
    rotation: i32,
    beacons: Vec<Pos3d>,
}

impl fmt::Debug for RotatedBeacons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "scanner {} with rotation {} ({} beacons)",
            self.scan_number,
            self.rotation,
            self.beacons.len()
        )
    }
}

// returns (set of beacons, vec of scanner positions)
fn search(scans: &[Scan]) -> (HashSet<Pos3d>, Vec<Pos3d>) {
    // everything will be relative to scan[0], so load its beacons into the
    // map straight away
    let mut beacons = HashSet::new();
    let mut scanners: Vec<Pos3d> = scans.iter().map(|_| ORIGIN3D).collect();
    for beacon in &scans[0].beacons {
        beacons.insert(beacon.clone());
    }

    // queue of (scan_number, rot, rotated_beacons) for each scan/rotation pair
    let mut queue: VecDeque<RotatedBeacons> = VecDeque::new();
    for scan in scans.iter().skip(1) {
        for rot in 0..24 {
            queue.push_back(RotatedBeacons {
                scan_number: scan.number,
                rotation: rot,
                beacons: scan.beacons.iter().map(|p| p.rotate(rot)).collect(),
            });
        }
    }

    // queue of known sets of points (re-orientated)
    let mut known: Vec<HashSet<Pos3d>> =
        vec![HashSet::from_iter(scans[0].beacons.clone().into_iter())];

    while let Some(rotated_beacons) = queue.pop_front() {
        let mut matched = false;
        // println!(
        //     "Trying to match {:?}, {} unknown sets, {} known sets, {} known beacons",
        //     rotated_beacons,
        //     queue.len() + 1,
        //     known.len(),
        //     beacons.len()
        // );
        for k in known.iter() {
            if let Some(scanner_pos) = match_beacons(&rotated_beacons, k) {
                // println!("MATCHED!  {:?} is at {:?}", rotated_beacons, scanner_pos);
                matched = true;
                // merge everything in this match into the set of known beacons
                let set: HashSet<Pos3d> =
                    HashSet::from_iter(rotated_beacons.beacons.iter().map(|p| p + &scanner_pos));
                beacons.extend(set.clone());
                // store this set for later comparisons
                known.push(set);
                // record the scanner's position for part 2
                scanners[rotated_beacons.scan_number] = scanner_pos;
                // remove other rotations of this scan from the queue
                queue.retain(|rb| rb.scan_number != rotated_beacons.scan_number);
                break;
            }
        }
        if !matched {
            // try this one later
            queue.push_back(rotated_beacons);
        }
    }

    (beacons, scanners)
}

// assuming "other" is already correctly orientated list of beacons.
// for a given (pre-rotated) set of beacons, try shifting each beacon
// to the target beacons in turn to see if an offset matches enough known beacons
// to be considered the correct rotation/offset.
//
// if found, returns the deduced scanner position.  otherwise returns None.
fn match_beacons(rb: &RotatedBeacons, set: &HashSet<Pos3d>) -> Option<Pos3d> {
    // now try to guess the offset.  any pos in 'beacons' might map to any pos in `set`
    // but if it's not found by the time only 11 are left to check, this rotation will
    // not match.
    for i in 0..rb.beacons.len() - 11 {
        for known in set.iter() {
            let offset = known - &rb.beacons[i];
            let mut count = 0;
            for b in rb.beacons.iter().map(|p| p + &offset) {
                if set.contains(&b) {
                    if count == 11 {
                        return Some(offset);
                    }
                    count += 1;
                }
            }
        }
    }
    None
}

fn max_distance(scanners: &[Pos3d]) -> u32 {
    let mut max = 0;

    for (i, p1) in scanners.iter().enumerate() {
        for p2 in scanners.iter().skip(i + 1) {
            let dist = p1.manhattan_distance(p2);
            if dist > max {
                max = dist;
            }
        }
    }

    max
}

#[test]
fn test() {
    let test_input = "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14
";
    let scans = parse_scans(test_input);
    let (beacons, scanners) = search(&scans);
    assert_eq!(79, beacons.len());
    assert_eq!(3621, max_distance(&scanners));
}
