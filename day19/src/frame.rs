pub(crate)
use std::fmt::Display;
use std::fmt::Formatter;
use crate::Point;

pub struct FrameOfReference {
    pub position: Point,
    pub orientation: Orientation
}

pub struct Orientation {
    pub facing: Direction,
    pub up: Direction
}

#[derive(Clone)]
pub enum Direction {
    X(bool),
    Y(bool),
    Z(bool)
}

impl Orientation {
    const BASE: Orientation = Self {
        facing: Direction::X(true),
        up: Direction::Y(true)
    };

    fn new(facing: Direction, up: Direction) -> Result<Self, String> {
        let same_axis = match (&facing, &up) {
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

    pub fn all() -> Vec<Self> {
        let mut vec = Vec::new();
        for f in Direction::all() {
            for u in Direction::all() {
                if let Ok(orientation) = Self::new(f.clone(), u) {
                    vec.push(orientation);
                }
            }
        }
        vec
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Facing: {}, Up: {}", self.facing, self.up)
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

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Direction::X(positive) => write!(f, "X{}", if *positive { "+" } else { "-" }),
            Direction::Y(positive) => write!(f, "Y{}", if *positive { "+" } else { "-" }),
            Direction::Z(positive) => write!(f, "Z{}", if *positive { "+" } else { "-" })
        }
    }
}