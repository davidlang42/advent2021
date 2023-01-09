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
        print_path(&grid);
        let big_grid = grid.expand(5, 5);
        print_path(&big_grid);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn print_path(grid: &Grid) {
    let start = Point { x: 0, y: 0 };
    let end = Point { x: grid.width - 1, y: grid.height - 1 };
    let path = astar(
        &start,
        |p| p.adjacent_risks(&grid),
        |p| p.distance(&end),
        |p| *p == end
    ).expect("No path found");
    println!("In {}x{} grid, it takes {} steps with total risk of {}", grid.height, grid.width, path.0.len(), path.1);
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let risk: Vec<Vec<u8>> = text.lines().map(|l| l.chars().map(|c| c as u8 - '0' as u8).collect()).collect();
        Ok(Grid::new(risk))
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

    fn new(risk: Vec<Vec<u8>>) -> Self {
        Self {
            height: risk.len(),
            width: risk[0].len(),
            risk
        }
    }

    fn expand(&self, scale_x: u8, scale_y: u8) -> Grid {
        let mut new_risk = Vec::new();
        for y in 0..self.height {
            let mut new_row = Vec::new();
            for dx in 0..scale_x {
                for x in 0..self.width {
                    new_row.push(Self::increase_risk(self.risk[y][x], dx));
                }
            }
            new_risk.push(new_row);
        }
        for dy in 1..scale_y {
            for y in 0..self.height {
                let mut new_row = Vec::new();
                for x in 0..new_risk[0].len() {
                    new_row.push(Self::increase_risk(new_risk[y][x], dy))
                }
                new_risk.push(new_row);
            }
        }
        Grid::new(new_risk)
    }

    fn increase_risk(risk_level: u8, increase_by: u8) -> u8 {
        let mut new_level = risk_level + increase_by;
        while new_level > 9 {
            new_level -= 9;
        }
        new_level
    }
}