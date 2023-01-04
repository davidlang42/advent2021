use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<isize> = text.split(",").map(|n| n.parse().unwrap()).collect();
        let mut fish: HashMap<isize, usize> = HashMap::new(); // map from fish countdown number to count at that number
        for n in numbers {
            add_value(&mut fish, n, 1);
        }
        for i in 0..256 {
            fish = simulate(&fish);
            println!("Day #{} fish: {}", i+1, fish.values().sum::<usize>());
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn simulate(fish: &HashMap<isize, usize>) -> HashMap<isize, usize> {
    let mut new_fish = HashMap::new();
    for (old_fish, count) in fish {
        if *old_fish == 0 {
            add_value(&mut new_fish, 8, *count);
            add_value(&mut new_fish, 6, *count);
        } else {
            add_value(&mut new_fish, old_fish - 1, *count);
        }
    }
    new_fish
}

fn add_value(map: &mut HashMap<isize, usize>, key: isize, delta: usize) {
    if let Some(existing) = map.get(&key) {
        map.insert(key, existing + delta);
    } else {
        map.insert(key, delta);
    }
}