use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut fish: Vec<isize> = text.split(",").map(|n| n.parse().unwrap()).collect();
        for i in 0..256 {
            let mut new_fish = simulate(&mut fish);
            fish.append(&mut new_fish);
            println!("Day #{} fish: {}", i+1, fish.len());
        }
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn simulate(fish: &mut Vec<isize>) -> Vec<isize> {
    let mut new_fish = Vec::new();
    for i in 0..fish.len() {
        if fish[i] == 0 {
            new_fish.push(8);
            fish[i] = 6;
        } else {
            fish[i] -= 1;
        }
    }
    new_fish
}