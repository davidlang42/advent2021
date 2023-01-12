pub(crate)
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::collections::HashSet;
use crate::Point;
use crate::FrameOfReference;

pub struct Scanner {
    name: String,
    frame: Option<FrameOfReference>,
    pub beacons: Vec<Point>//TODO
}

impl Scanner {
    // fn find_frame(&self, known: &Scanner, min_common_points: usize) -> Option<FrameOfReference> {
    //     if known.frame.is_none() {
    //         panic!("Cannot find a frame of reference without a known frame");
    //     }
    //     for orientation in Orientation::all() {
    //         if let Some(position) = find_common_points(known, self, min_common_points, &orientation) {
    //             return Some((position, orientation));
    //         }
    //     }
    //     None
    // }

    // fn find_common_points(source: &Scanner, unknown: &Scanner, min_common_points: usize, orientation: &Orientation) -> Option<Point> {
    //     let possible_positions = find_all_offsets(&source.beacons, &unknown.beacons, orientation);
    //     for position in possible_positions {
    //         let mut count = 0;
    //         for sb in &source.beacons {
    //             let sb_absolute = sb.absolute(&source.position.unwrap(), &source.orientation.unwrap());
    //             for ub in &unknown.beacons {
    //                 if sb_absolute == ub.absolute(&position, orientation) {
    //                     count += 1;
    //                     break;
    //                 }
    //             }
    //             if count >= min_common_points {
    //                 return Some(position);
    //             }
    //         }
    //     }
    //     None
    // }

    // fn find_all_offsets(a_vec: &Vec<Point>, b_vec: &Vec<Point>, _orientation: &Orientation) -> HashSet<Point> {
    //     let mut set = HashSet::new();
    //     for a in a_vec {
    //         for b in b_vec {
    //             //TODO handle orientation
    //             set.insert(a.offset(b, _orientation));
    //         }
    //     }
    //     set
    // }

    fn absolute_beacons(&self) -> Vec<Point> {
        if let Some(frame) = &self.frame {
            self.beacons.iter().map(|b| b.absolute(frame)).collect()
        } else {
            panic!("Cannot find absolute beacon positions without a frame of reference");
        }
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
            frame: None,
            beacons
        })
    }
}

impl Display for Scanner {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}