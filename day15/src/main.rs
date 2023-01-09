use std::env;
use std::fs;
use std::str::FromStr;
use pathfinding::prelude::astar;

struct Grid {
    height: usize,
    width: usize,
    risk: Vec<Vec<u8>>
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let grid: Grid = text.parse().unwrap();
        let start = Point { x: 0, y: 0 };
        let end = Point { x: grid.width - 1, y: grid.height - 1 };
        let path = astar(
            &start,
            |p| p.adjacent_risks(&grid),
            |p| p.distance(&end),
            |p| *p == end
        ).expect("No path found");
        println!("{} steps with total risk of {}", path.0.len(), path.1);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let risk: Vec<Vec<u8>> = text.lines().map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect()).collect();
        Ok(Grid {
            height: risk.len(),
            width: risk[0].len(),
            risk
        })
    }
}

impl Point {
    fn adjacent(&self, grid_height: usize, grid_width: usize) -> Vec<Point> {
        let mut points = Vec::new();
        if self.x > 0 {
            points.push(Point { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            points.push(Point { x: self.x, y: self.y - 1 });
        }
        if self.x < grid_width - 1 {
            points.push(Point { x: self.x + 1, y: self.y });
        }
        if self.y < grid_height - 1 {
            points.push(Point { x: self.x, y: self.y + 1 });
        }
        points
    }

    fn adjacent_risks(&self, grid: &Grid) -> Vec<(Point, usize)> {
        self.adjacent(grid.height, grid.width).into_iter().map(|p| {
            let risk = grid.get_risk(&p) as usize;
            (p, risk)
        }).collect()
    }

    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Grid {
    fn get_risk(&self, p: &Point) -> u8 {
        self.risk[p.y][p.x]
    }
}