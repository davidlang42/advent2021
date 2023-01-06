use std::env;
use std::fs;
use std::str::FromStr;

struct Map {
    values: Vec<Vec<u32>>,
    width: isize,
    height: isize
}

struct Point {
    x: isize,
    y: isize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        let points = map.find_low_points();
        let sum: u32 = points.iter().map(|p| map.get_value(p).unwrap() + 1).sum();
        println!("{} low points with a total risk of {}", points.len(), sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let values: Vec<Vec<u32>> = text.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
        Ok(Map {
            height: values.len() as isize,
            width: values[0].len() as isize,
            values
        })
    }
}

impl Map {
    fn get_value(&self, point: &Point) -> Option<u32> {
        if point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height {
            None
        } else {
            Some(self.values[point.y as usize][point.x as usize])
        }
    }

    fn find_low_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point { x: x, y };
                let v = self.get_value(&p).unwrap();
                let adjacent = vec![p.up(), p.left(), p.right(), p.down()];
                if adjacent.iter().all(|a| match self.get_value(a) { Some(a_v) => v < a_v, None => true }) {
                    points.push(p);
                }
            }
        }
        points
    }
}

impl Point {
    fn up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1
        }
    }

    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1
        }
    }

    fn left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y
        }
    }

    fn right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y
        }
    }
}