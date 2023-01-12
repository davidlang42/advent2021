use std::env;
use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

struct Image {
    pixels: HashMap<Point, bool>,
    edge: bool
}

struct Enhancer {
    data: [bool; 512]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut iter = text.split("\r\n\r\n");
        let enhancer: Enhancer = iter.next().unwrap().parse().unwrap();
        let mut image: Image = iter.next().unwrap().parse().unwrap();
        let cycles: usize = args[2].parse().unwrap();
        println!("Image starting lit pixels: {}", image.lit_pixels());
        for c in 0..cycles {
            image = enhancer.enhance(&image);
            println!("Image lit pixels after enhancement #{}: {}", c+1, image.lit_pixels());
        }
    } else {
        println!("Please provide 2 arguments: Filename, Enhancement Cycles");
    }
}

impl FromStr for Enhancer {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let data: [bool; 512] = line.chars().map(|c| c == '#').collect::<Vec<bool>>().try_into().unwrap();
        Ok(Self { data })
    }
}

impl FromStr for Image {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut pixels = HashMap::new();
        for (y, line) in text.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                pixels.insert(Point {
                    x: x as isize,
                    y: y as isize
                }, c == '#');
            }
        }
        Ok(Self {
            pixels,
            edge: false
        })
    }
}

impl Image {
    fn lit_pixels(&self) -> usize {
        self.pixels.values().filter(|&v| *v).count()
    }

    fn bounds(&self) -> (Point, Point) {
        (Point {
            x: self.pixels.keys().map(|p| p.x).min().unwrap(),
            y: self.pixels.keys().map(|p| p.y).min().unwrap()
        },
        Point {
            x: self.pixels.keys().map(|p| p.x).max().unwrap(),
            y: self.pixels.keys().map(|p| p.y).max().unwrap()
        })
    }

    fn get(&self, point: &Point) -> bool {
        if let Some(value) = self.pixels.get(point) {
            *value
        } else {
            self.edge
        }
    }
}

impl Enhancer {
    fn enhance(&self, image: &Image) -> Image {
        let mut new_pixels = HashMap::new();
        let (min, max) = image.bounds();
        for x in (min.x-1)..(max.x+2) {
            for y in (min.y-1)..(max.y+2) {
                let p = Point { x, y };
                let binary_index: String = p.adjacent().iter().map(|p| if image.get(p) { '1' } else { '0' }).collect();
                let decimal_index = usize::from_str_radix(&binary_index, 2).unwrap();
                new_pixels.insert(p, self.data[decimal_index]);
            }
        }
        let new_edge = if image.edge {
            self.data[511]
        } else {
            self.data[0]
        };
        Image {
            pixels: new_pixels,
            edge: new_edge
        }
    }
}

impl Point {
    fn adjacent(&self) -> Vec<Point> {
        vec![
            Point { x: self.x-1, y: self.y-1 },
            Point { x: self.x, y: self.y-1 },
            Point { x: self.x+1, y: self.y-1 },
            Point { x: self.x-1, y: self.y },
            Point { x: self.x, y: self.y },
            Point { x: self.x+1, y: self.y },
            Point { x: self.x-1, y: self.y+1 },
            Point { x: self.x, y: self.y+1 },
            Point { x: self.x+1, y: self.y+1 }
        ]
    }
}