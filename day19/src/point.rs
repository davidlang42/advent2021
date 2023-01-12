pub(crate)
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;
use crate::frame::FrameOfReference;
use crate::Orientation;
use crate::frame::Direction;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize
}

impl Point {
    pub fn absolute(&self, frame: &FrameOfReference) -> Point {
        let rotated = self.rotate(&frame.orientation);
        Point {
            x: frame.position.x + rotated.x,
            y: frame.position.y + rotated.y,
            z: frame.position.z + rotated.z
        }
    }

    fn rotate(&self, from: &Orientation) -> Point {
        // rotate around 0,0,0 into BASE orientation
        match from.up {
            Direction::Y(_) => {
                match from.facing {
                    Direction::X(true) => *self,
                    Direction::Z(true) => self.y_left(),
                    Direction::X(false) => self.y_left().y_left(),
                    Direction::Z(false) => self.y_left().y_left().y_left(),
                    _ => panic!("Invalid orientation")
                }
            },
            Direction::X(b) => {
                let step = match from.facing {
                    Direction::Y(false) => *self,
                    Direction::Z(true) => self.x_left(),
                    Direction::Y(true) => self.x_left().x_left(),
                    Direction::Z(false) => self.x_left().x_left().x_left(),
                    _ => panic!("Invalid orientation")
                }.z_left();
                if b {
                    step
                } else {
                    step.z_left().z_left()
                }
            }
            Direction::Z(b) => {
                let step = match from.facing {
                    Direction::X(true) => *self,
                    Direction::Y(false) => self.z_left(),
                    Direction::X(false) => self.z_left().z_left(),
                    Direction::Y(true) => self.z_left().z_left().z_left(),
                    _ => panic!("Invalid orientation")
                }.x_left();
                if b {
                    step.x_left().x_left()
                } else {
                    step
                }
            }
        }
    }

    fn x_left(&self) -> Point {
        Point {
            x: self.x,
            y: -self.z,
            z: self.y
        }
    }

    fn y_left(&self) -> Point {
        Point {
            x: self.z,
            y: self.y,
            z: -self.x
        }
    }

    fn z_left(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
            z: self.z
        }
    }

    pub fn offset(&self, absolute_position: &Point, _reference_orientation: &Orientation) -> Point {
        todo!();
        // Point {
        //     x: absolute_position.x - self.x,
        //     y: absolute_position.y - self.y,
        //     z: absolute_position.z - self.z
        // }
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