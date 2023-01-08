use std::env;
use std::fs;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::HashSet;

struct Cave {
    name: String,
    size: CaveSize
}

#[derive(PartialEq)]
enum CaveSize {
    Big,
    Small
}

struct System {
    caves: HashMap<String, Cave>,
    connections: HashMap<String, HashSet<String>>
}

struct Connection(String, String);

#[derive(Clone)]
struct Path {
    order: Vec<String>,
    visited: HashSet<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let connections: Vec<Connection> = text.lines().map(|l| l.parse().unwrap()).collect();
        let system = System::new(&connections);
        let paths = system.find_all_paths("start", "end", &Path::new());
        println!("Found {} paths", paths.len());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = line.split("-").collect();
        if segments.len() == 2 {
            Ok(Self(segments[0].to_string(), segments[1].to_string()))
        } else {
            Err(format!("Expected 2 segments: {}", line))
        }
    }
}

impl System {
    fn new(connections: &Vec<Connection>) -> Self {
        let mut system = System {
            caves: HashMap::new(),
            connections: HashMap::new()
        };
        for connection in connections {
            system.add_cave(&connection.0);
            system.add_cave(&connection.1);
        }
        for connection in connections {
            system.add_connection(&connection.0, &connection.1);
            system.add_connection(&connection.1, &connection.0);
        }
        system
    }

    fn add_cave(&mut self, name: &str) -> bool {
        if self.caves.contains_key(name) {
            false
        } else {
            self.caves.insert(name.to_string(), Cave::new(name.to_string()));
            self.connections.insert(name.to_string(), HashSet::new());
            true
        }
    }

    fn add_connection(&mut self, from: &str, to: &str) {
        let from_cave = self.caves.get(from).unwrap();
        let to_cave = self.caves.get(to).unwrap();
        if from_cave.size == CaveSize::Big && to_cave.size == CaveSize::Big {
            panic!("Cannot connect big cave {} to big cave {}", from, to);
        }
        let cave_connections = self.connections.get_mut(from).unwrap();
        cave_connections.insert(to.to_string());
    }

    fn find_all_paths(&self, from: &str, to: &str, base_path: &Path) -> Vec<Path> {
        let mut path = base_path.clone();
        if from == to {
            return vec![path]; // this path is complete
        }
        let mut paths = Vec::new();
        let cave = self.caves.get(from).unwrap();
        if !path.visit(cave) {
            return paths; // no valid paths
        }
        for next in self.connections.get(from).unwrap() {
            paths.append(&mut self.find_all_paths(next, to, &path));
        }
        paths
    }
}

impl Cave {
    fn new(name: String) -> Self {
        let size = if name.chars().next().unwrap().is_ascii_uppercase() {
            CaveSize::Big
        } else {
            CaveSize::Small
        };
        Self { name, size }
    }
}

impl Path {
    fn new() -> Self {
        Self {
            order: Vec::new(),
            visited: HashSet::new()
        }
    }

    fn visit(&mut self, next: &Cave) -> bool { // returns true if valid
        if next.size == CaveSize::Big || self.visited.insert(next.name.to_string()) { // big caves can be visited more than once
            self.order.push(next.name.to_string());
            true
        } else {
            false
        }
    }
}