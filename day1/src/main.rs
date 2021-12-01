use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<u32> = text.split("\r\n").map(|s| s.parse()
            .expect(&format!("Error parsing number {}", s))).collect();
        println!("Increasing by 1: {}", count_increasing(&numbers, 1));
        println!("Increasing by 3: {}", count_increasing(&numbers, 3));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn count_increasing(list: &Vec<u32>, by: usize) -> u32 {
    let mut count: u32 = 0;
    let mut previous: u32 = 0;
    for i in by-1..list.len() {
        let mut value: u32 = 0;
        for offset in 0..by {
            value += list[i-offset];
        }
        if value > previous {
            count += 1;
        }
        previous = value;
    }
    return count - 1;
}