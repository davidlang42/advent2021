use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

struct Paper {
    dots: HashSet<Point>
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

enum Fold {
    Horizontal { y: usize },
    Vertical { x: usize }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let segments: Vec<&str> = text.split("\r\n\r\n").collect();
        let mut paper: Paper = segments[0].parse().unwrap();
        let folds: Vec<Fold> = segments[1].lines().map(|l| l.parse().unwrap()).collect();
        println!("Initial dots: {}", paper.dots.len());
        paper.fold(&folds[0]);
        println!("After 1 fold: {}", paper.dots.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Paper {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let dots = text.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { dots })
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<usize> = line.split(",").map(|n| n.parse().unwrap()).collect();
        if numbers.len() == 2 {
            Ok(Self {
                x: numbers[0],
                y: numbers[1]
            })
        } else {
            Err(format!("Expected 2 numbers: {}", line))
        }
    }
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split("=").collect();
        if parts.len() == 2 {
            match parts[0] {
                "fold along y" => Ok(Fold::Horizontal { y: parts[1].parse().unwrap() }),
                "fold along x" => Ok(Fold::Vertical { x: parts[1].parse().unwrap() }),
                _ => Err(format!("Invalid fold instruction: {}", parts[0]))
            }
        } else {
            Err(format!("Expected 2 parts: {}", line))
        }
    }
}

impl Paper {
    fn fold(&mut self, fold: &Fold) {
        let mut new_dots = HashSet::new();
        for p in &self.dots {
            new_dots.insert(match fold {
                Fold::Horizontal { y } if p.y > *y => Point { x: p.x, y: 2*y - p.y },
                Fold::Vertical { x } if p.x > *x => Point { x: 2*x - p.x, y: p.y },
                _ => *p
            });
        }
        self.dots = new_dots;
    }
}