use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: isize,
    y: isize
}

struct Line {
    from: Point,
    to: Point
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let lines: Vec<Line> = text.lines().map(|l| l.parse().unwrap()).collect();
        let simple = lines.iter().filter(|l| l.horizontal() || l.vertical()).collect();
        let simple_overlaps = find_overlaps(&simple);
        println!("Orthogonal lines only: Found {} overlapping points", simple_overlaps.len());
        let all = lines.iter().collect();
        let all_overlaps = find_overlaps(&all);
        println!("Include diagonals: Found {} overlapping points", all_overlaps.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = line.split(" -> ").map(|p| p.parse().unwrap()).collect();
        if points.len() == 2 {
            Ok(Line {
                from: points[0],
                to: points[1]
            })
        } else {
            Err(format!("Expected 2 points: {}", line))
        }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        let values: Vec<isize> = point.split(",").map(|v| v.parse().unwrap()).collect();
        if values.len() == 2 {
            Ok(Point {
                x: values[0],
                y: values[1]
            })
        } else {
            Err(format!("Expected 2 values: {}", point))
        }
    }
}

impl Line {
    fn horizontal(&self) -> bool {
        self.from.x == self.to.x
    }

    fn vertical(&self) -> bool {
        self.from.y == self.to.y
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let (dx, dy) = if self.horizontal() {
            (0, (self.to.y-self.from.y).signum())
        } else if self.vertical() {
            ((self.to.x-self.from.x).signum(), 0)
        } else {
            ((self.to.x-self.from.x).signum(), (self.to.y-self.from.y).signum())
        };
        let mut p = self.from;
        loop {
            points.push(p);
            if p == self.to {
                break;
            }
            p.x += dx;
            p.y += dy;
        }
        points
    }
}

fn find_overlaps(lines: &Vec<&Line>) -> HashSet<Point> {
    let mut taken = HashSet::new();
    let mut overlaps = HashSet::new();
    for line in lines {
        for point in line.points() {
            if !taken.insert(point) {
                overlaps.insert(point);
            }
        }
    }
    overlaps
}