pub(crate)
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use std::collections::HashSet;
use crate::Point;
use crate::FrameOfReference;
use crate::Orientation;

pub struct Scanner {
    name: String,
    pub frame: Option<FrameOfReference>,
    beacons: Vec<Point>
}

impl Scanner {
    pub fn find_frame(&self, known: &Scanner, min_common_points: usize) -> Option<FrameOfReference> {
        if let Some(known_frame) = &known.frame {
            for orientation in Orientation::all() {
                let possible_positions = known.find_all_offsets(&self.beacons, &orientation);
                for position in possible_positions {
                    let potential_frame = FrameOfReference { position, orientation: orientation.clone() };
                    let mut count = 0;
                    for kb in &known.beacons {
                        let kb_absolute = kb.absolute(known_frame);
                        for ub in &self.beacons {
                            if kb_absolute == ub.absolute(&potential_frame) {
                                count += 1;
                                break;
                            }
                        }
                        if count >= min_common_points {
                            return Some(potential_frame);
                        }
                    }
                }
            }
            None
        } else {
            panic!("Cannot find a frame without a known frame of reference");
        }
    }

    fn find_all_offsets(&self, unknown: &Vec<Point>, orientation: &Orientation) -> HashSet<Point> {
        let mut set = HashSet::new();
        for k in self.absolute_beacons() {
            for u in unknown {
                set.insert(k.offset(u, orientation));
            }
        }
        set
    }

    pub fn absolute_beacons(&self) -> Vec<Point> {
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