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
        let result = count_increasing(numbers);
        println!("Result: {}", result);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn count_increasing(list: Vec<u32>) -> u32 {
    let mut previous: u32 = 0;
    let mut count: u32 = 0;
    for (_, value) in list.iter().enumerate() {
        if value > &previous {
            count += 1;
        }
        previous = *value;
    }
    return count - 1;
}