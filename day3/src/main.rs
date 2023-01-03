use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let numbers: Vec<Vec<usize>>  = text.lines().map(|l| l.chars().map(|c| c as usize - '0' as usize).collect()).collect();
        let mut most = Vec::new();
        let mut least = Vec::new();
        for i in 0..numbers[0].len() {
            let sum: usize = numbers.iter().map(|n| n[i]).sum();
            if sum > numbers.len() / 2 {
                most.push('1');
                least.push('0');
            } else {
                most.push('0');
                least.push('1');
            }
        }
        let gamma = usize::from_str_radix(&most.iter().collect::<String>(), 2).unwrap();
        let epsilon = usize::from_str_radix(&least.iter().collect::<String>(), 2).unwrap();
        println!("{:?} x {:?} = {}", most, least, gamma*epsilon);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}