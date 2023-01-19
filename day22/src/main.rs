#![feature(hash_drain_filter)]
use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashSet;

struct RebootStep {
    value: bool,
    cubeoid: Cubeoid
}

#[derive(Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Eq, Hash, PartialEq)]
struct Cubeoid {
    min: Point,
    max: Point
}

struct Reactor {
    cubeoids: HashSet<Cubeoid>
}

#[cfg(test)]
mod tests;

impl FromStr for Cubeoid {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<&str> = line.split(",").collect();
        if coordinates.len() != 3 {
            return Err(format!("Expected 3 coordinates: {}", line));
        }
        let mut min = Point::new();
        let mut max = Point::new();
        (min.x, max.x) = Self::parse_range(coordinates[0])?;
        (min.y, max.y) = Self::parse_range(coordinates[1])?;
        (min.z, max.z) = Self::parse_range(coordinates[2])?;
        Ok(Self {
            min,
            max
        })
    }
}

impl Cubeoid {
    fn parse_range(assignment: &str) -> Result<(isize, isize), <Self as FromStr>::Err> {
        let range: Vec<&str> = assignment.split("=").collect();
        if range.len() != 2 {
            return Err(format!("Expected single assignment in: {}", assignment));
        }
        let values: Vec<&str> = range[1].split("..").collect();
        if values.len() != 2 {
            return Err(format!("Expected 2 values: {}", range[1]));
        }
        Ok((values[0].parse().unwrap(), values[1].parse().unwrap()))
    }

    fn count_cubes(&self, limit: &Option<usize>) -> usize {
        Self::limited_range(self.min.x, self.max.x, limit) * Self::limited_range(self.min.y, self.max.y, limit) * Self::limited_range(self.min.z, self.max.z, limit)
    }

    fn limited_range(mut min: isize, mut max: isize, limit: &Option<usize>) -> usize {
        if let Some(l) = limit {
            if min < -(*l as isize) {
                min = -(*l as isize);
                if max < -(*l as isize) {
                    return 0;
                }
            }
            if max > *l as isize {
                max = *l as isize;
                if min > *l as isize {
                    return 0;
                }
            }
        }
        (max - min + 1).try_into().unwrap()
    }

    fn overlaps(&self, other: &Cubeoid) -> bool {
        Self::overlap_1d(self.min.x, self.max.x, other.min.x, other.max.x)
            && Self::overlap_1d(self.min.y, self.max.y, other.min.y, other.max.y)
            && Self::overlap_1d(self.min.z, self.max.z, other.min.z, other.max.z)
    }

    fn overlap_1d(a_min: isize, a_max: isize, b_min: isize, b_max: isize) -> bool {
        a_max >= b_min && b_max >= a_min
    }

    fn subtract(&self, other: &Cubeoid) -> Vec<Cubeoid> {
        let mut new_cubeoids = Vec::new();
        for (x_min, x_max) in Self::segments_1d(self.min.x, self.max.x, other.min.x, other.max.x) {
            for (y_min, y_max) in Self::segments_1d(self.min.y, self.max.y, other.min.y, other.max.y) {
                for (z_min, z_max) in Self::segments_1d(self.min.z, self.max.z, other.min.z, other.max.z) {
                    let new_cubeoid = Cubeoid {
                        min: Point {
                            x: x_min,
                            y: y_min,
                            z: z_min
                        },
                        max: Point {
                            x: x_max,
                            y: y_max,
                            z: z_max
                        }
                    };
                    if !new_cubeoid.overlaps(other) {
                        new_cubeoids.push(new_cubeoid);
                    }
                }
            }
        }
        new_cubeoids
    }

    fn segments_1d(a_min: isize, a_max: isize, b_min: isize, b_max: isize) -> Vec<(isize, isize)> {
        let mut edges = vec![a_min];
        if b_min <= a_max && b_min >= a_min {
            edges.push(b_min);
        }
        if b_max <= a_max && b_max >= a_min {
            edges.push(b_max + 1);
        }
        edges.push(a_max + 1);
        let mut segments = Vec::new();
        for i in 1..edges.len() {
            let left = edges[i-1];
            let right = edges[i]-1;
            if left <= right {
                segments.push((left, right));
            }
        }
        segments
    }
}

impl FromStr for RebootStep {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() != 2 {
            return Err(format!("Expected 2 parts: {}", line));
        }
        Ok(Self {
            value: parts[0] == "on",
            cubeoid: parts[1].parse()?
        })
    }
}

impl Point {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0
        }
    }
}

impl Reactor {
    fn new() -> Self {
        Self {
            cubeoids: HashSet::new()
        }
    }

    fn count_cubes(&self, limit: &Option<usize>) -> usize {
        self.cubeoids.iter().map(|c| c.count_cubes(limit)).sum()
    }

    fn set(&mut self, value: bool, cubeoid: Cubeoid) {
        let conflicts: Vec<Cubeoid> = self.cubeoids.drain_filter(|c| c.overlaps(&cubeoid)).collect();
        for conflict in conflicts {
            let replacements = conflict.subtract(&cubeoid);
            for replacement in replacements {
                self.cubeoids.insert(replacement);
            }
        }
        if value {
            self.cubeoids.insert(cubeoid);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let steps: Vec<RebootStep> = text.lines().map(|l| l.parse().unwrap()).collect();
        let mut reactor = Reactor::new();
        for step in steps {
            reactor.set(step.value, step.cubeoid);
        }
        println!("In -50..50, {} cubes are on", reactor.count_cubes(&Some(50)));
        println!("Overall, {} cubes are on", reactor.count_cubes(&None));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
