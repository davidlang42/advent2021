use std::env;
use std::fs;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

struct Scanner {
    name: String,
    position: Option<Point>,
    orientation: Option<Orientation>,
    beacons: Vec<Point>
}

#[derive(Copy, Clone)]
struct Orientation {
    facing: Direction,
    up: Direction
}

#[derive(Copy, Clone)]
enum Direction {
    X(bool),
    Y(bool),
    Z(bool)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut scanners = text.split("\r\n\r\n").map(|s| s.parse().unwrap());
        //TODO
        let s: Scanner = scanners.next().unwrap();
        for o in Orientation::all() {
            for p in &s.beacons {
                println!("{}", p.absolute(&Point { x: 0, y: 0, z: 0 }, &o));
            }
            println!("");
        }

        return;
        //TODO
        let minimum_overlap = 12;
        let mut reference_scanner: Scanner = scanners.next().unwrap();
        reference_scanner.position = Some(Point { x: 0, y: 0, z: 0 });
        reference_scanner.orientation = Some(Orientation::BASE);
        let mut found: Vec<Scanner> = vec![reference_scanner];
        let mut remaining: Vec<Scanner> = scanners.collect();
        while remaining.len() > 0 {
            let mut matched = None;
            for source in &found {
                for i in 0..remaining.len() {
                    if let Some((position, orientation)) = find_matching_scanners(source, &remaining[i], minimum_overlap) {
                        let mut newly_found = remaining.remove(i);
                        newly_found.position = Some(position);
                        newly_found.orientation = Some(orientation);
                        println!("Matched {} with {}", newly_found, source);
                        matched = Some(newly_found);
                        break;
                    }
                }
                if matched.is_some() {
                    break;
                }
            }
            if let Some(newly_found) = matched {
                found.push(newly_found);
            } else {
                panic!("No matches found with {} remaining", remaining.len());
            }
        }
        let absolute_beacons: HashSet<Point> = found.iter().flat_map(|s| s.absolute_beacons()).collect();
        println!("Total beacons: {}", absolute_beacons.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut iter = text.lines();
        let name = iter.next().unwrap().to_string();
        let beacons: Vec<Point> = iter.map(|p| p.parse().unwrap()).collect();
        Ok(Self {
            name,
            position: None,
            orientation: None,
            beacons
        })
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<isize> = line.split(",").map(|c| c.parse().unwrap()).collect();
        if coordinates.len() == 3 {
            Ok(Point {
                x: coordinates[0],
                y: coordinates[1],
                z: coordinates[2]
            })
        } else {
            Err(format!("Expected 3 coordinates: {}", line))
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Direction::X(positive) => write!(f, "X{}", if *positive { "+" } else { "-" }),
            Direction::Y(positive) => write!(f, "Y{}", if *positive { "+" } else { "-" }),
            Direction::Z(positive) => write!(f, "Z{}", if *positive { "+" } else { "-" })
        }
    }
}

impl Orientation {
    const BASE: Orientation = Self {
        facing: Direction::X(true),
        up: Direction::Y(true)
    };

    fn new(facing: Direction, up: Direction) -> Result<Self, String> {
        let same_axis = match (facing, up) {
            (Direction::X(_), Direction::X(_)) => true,
            (Direction::Y(_), Direction::Y(_)) => true,
            (Direction::Z(_), Direction::Z(_)) => true,
            _ => false
        };
        if same_axis {
            Err(format!("Cannot have orientation with facing ({}) and up ({}) on the same axis", facing, up))
        } else {
            Ok(Self { facing, up })
        }
    }

    fn all() -> Vec<Self> {
        let mut vec = Vec::new();
        for f in Direction::all() {
            for u in Direction::all() {
                if let Ok(orientation) = Self::new(f, u) {
                    vec.push(orientation);
                }
            }
        }
        vec
    }
}

impl Direction {
    fn all() -> Vec<Self> {
        vec![
            Direction::X(true),
            Direction::X(false),
            Direction::Y(true),
            Direction::Y(false),
            Direction::Z(true),
            Direction::Z(false)
        ]
    }
}

impl Scanner {
    fn absolute_beacons(&self) -> Vec<Point> {
        self.beacons.iter().map(|b| b.absolute(&self.position.unwrap(), &self.orientation.unwrap())).collect()
    }
}

impl Point {
    fn absolute(&self, reference_position: &Point, _reference_orientation: &Orientation) -> Point {
        //TODO handle orientation
        Point {
            x: reference_position.x + self.x,
            y: reference_position.y + self.y,
            z: reference_position.z + self.z
        }
    }

    fn offset(&self, absolute_position: &Point, _reference_orientation: &Orientation) -> Point {
        //TODO handle orientation
        Point {
            x: absolute_position.x - self.x,//TODO
            y: absolute_position.y - self.y,
            z: absolute_position.z - self.z
        }
    }
}

fn find_matching_scanners(source: &Scanner, unknown: &Scanner, min_common_points: usize) -> Option<(Point, Orientation)> {
    for orientation in Orientation::all() {
        if let Some(position) = find_common_points(source, unknown, min_common_points, &orientation) {
            return Some((position, orientation));
        }
    }
    None
}

fn find_common_points(source: &Scanner, unknown: &Scanner, min_common_points: usize, orientation: &Orientation) -> Option<Point> {
    let possible_positions = find_all_offsets(&source.beacons, &unknown.beacons, orientation);
    for position in possible_positions {
        let mut count = 0;
        for sb in &source.beacons {
            let sb_absolute = sb.absolute(&source.position.unwrap(), &source.orientation.unwrap());
            for ub in &unknown.beacons {
                if sb_absolute == ub.absolute(&position, orientation) {
                    count += 1;
                    break;
                }
            }
            if count >= min_common_points {
                return Some(position);
            }
        }
    }
    None
}

fn find_all_offsets(a_vec: &Vec<Point>, b_vec: &Vec<Point>, _orientation: &Orientation) -> HashSet<Point> {
    let mut set = HashSet::new();
    for a in a_vec {
        for b in b_vec {
            //TODO handle orientation
            set.insert(a.offset(b, _orientation));
        }
    }
    set
}